use std::cmp::Ordering;

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyIterator, PyModule, PyTuple, PyType};

use crate::Tree;
use crate::delete::delete_treap;
use crate::insert::insert_treap;
use crate::node::{IntervalTreeNode, Node};
use crate::search::search_interval;

/// Interval tree wrapper exposed to Python.
#[pyclass(name = "IntervalTree")]
pub struct PyIntervalTree {
    root: Tree<Py<PyAny>>,
}

impl PyIntervalTree {
    fn insert_internal(&mut self, left: u32, right: u32, data: Py<PyAny>) -> PyResult<()> {
        let node = Node::new(left, right, data).map_err(PyValueError::new_err)?;
        let tree_node = IntervalTreeNode::new(node).map_err(PyValueError::new_err)?;
        self.root = insert_treap(self.root.take(), Box::new(tree_node));
        Ok(())
    }
}

#[pymethods]
impl PyIntervalTree {
    #[new]
    fn new() -> Self {
        Self { root: None }
    }

    #[classmethod]
    fn from_tuples(_cls: &Bound<'_, PyType>, intervals: Bound<'_, PyAny>) -> PyResult<Self> {
        let mut tree = Self::new();
        let iter = PyIterator::from_object(&intervals)?;
        for item in iter {
            let item = item?;
            let (left, right, data) = parse_interval(&item)?;
            tree.insert_internal(left, right, data)?;
        }
        Ok(tree)
    }

    #[pyo3(signature = (interval))]
    fn insert(&mut self, interval: Bound<'_, PyAny>) -> PyResult<()> {
        let (left, right, data) = parse_interval(&interval)?;
        self.insert_internal(left, right, data)
    }

    #[pyo3(signature = (key))]
    fn delete(&mut self, key: Bound<'_, PyAny>) -> PyResult<bool> {
        let key = parse_key(&key)?;
        let existed = contains_key(&self.root, key);
        self.root = delete_treap(self.root.take(), key);
        Ok(existed)
    }

    #[pyo3(signature = (ql, qr, inclusive = true))]
    fn search<'py>(
        &'py self,
        ql: u32,
        qr: u32,
        inclusive: bool,
        py: Python<'py>,
    ) -> PyResult<Vec<(u32, u32, Py<PyAny>)>> {
        if ql > qr {
            return Err(PyValueError::new_err(
                "query range must have lower bound <= upper bound",
            ));
        }
        let mut out = Vec::new();
        if let Some(ref root) = self.root {
            for (left, right, data) in search_interval(root.as_ref(), ql, qr, inclusive) {
                out.push((left, right, data.clone_ref(py)));
            }
        }
        Ok(out)
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

fn parse_interval(obj: &Bound<'_, PyAny>) -> PyResult<(u32, u32, Py<PyAny>)> {
    let tuple = obj
        .downcast::<PyTuple>()
        .map_err(|_| PyTypeError::new_err("interval must be a tuple"))?;
    if tuple.len() != 3 {
        return Err(PyTypeError::new_err(
            "interval tuple must have exactly three items: (left, right, data)",
        ));
    }
    let left: u32 = tuple.get_item(0)?.extract()?;
    let right: u32 = tuple.get_item(1)?.extract()?;
    let data = tuple.get_item(2)?.unbind();
    Ok((left, right, data))
}

fn parse_key(obj: &Bound<'_, PyAny>) -> PyResult<(u32, u32)> {
    let tuple = obj
        .downcast::<PyTuple>()
        .map_err(|_| PyTypeError::new_err("key must be a tuple"))?;
    if tuple.len() != 2 {
        return Err(PyTypeError::new_err(
            "key tuple must have exactly two items: (left, right)",
        ));
    }
    let left: u32 = tuple.get_item(0)?.extract()?;
    let right: u32 = tuple.get_item(1)?.extract()?;
    Ok((left, right))
}

fn contains_key<T>(root: &Tree<T>, key: (u32, u32)) -> bool {
    let mut current = root.as_ref().map(|n| n.as_ref());
    while let Some(node) = current {
        match key.cmp(&(node.node.left, node.node.right)) {
            Ordering::Less => current = node.left_child.as_ref().map(|c| c.as_ref()),
            Ordering::Greater => current = node.right_child.as_ref().map(|c| c.as_ref()),
            Ordering::Equal => return true,
        }
    }
    false
}

#[pymodule]
pub fn intervaltree_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyIntervalTree>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::types::{PyList, PyTuple};
    use pyo3::{IntoPyObject, Python};

    #[test]
    fn python_interval_tree_crud() {
        Python::initialize();
        #[allow(deprecated)]
        Python::with_gil(|py| {
            let intervals = PyList::new(
                py,
                vec![(5u32, 10u32, "a"), (12u32, 18u32, "b"), (1u32, 4u32, "c")],
            )
            .expect("create list");
            let cls = py.get_type::<PyIntervalTree>();
            let mut tree =
                PyIntervalTree::from_tuples(&cls, intervals.into_any()).expect("build tree");
            assert!(!tree.is_empty());

            let hits = tree.search(0, 20, true, py).expect("search succeeds");
            assert_eq!(hits.len(), 3);

            let new_interval_items = vec![
                8u32.into_pyobject(py).unwrap().into_any().unbind(),
                11u32.into_pyobject(py).unwrap().into_any().unbind(),
                "d".into_pyobject(py).unwrap().into_any().unbind(),
            ];
            let new_interval = PyTuple::new(py, &new_interval_items).expect("tuple");
            tree.insert(new_interval.into_any()).expect("insert works");
            let hits = tree.search(9, 10, true, py).expect("search after insert");
            assert!(hits.iter().any(|(l, r, _)| (*l, *r) == (8, 11)));

            let key_items = vec![
                12u32.into_pyobject(py).unwrap().into_any().unbind(),
                18u32.into_pyobject(py).unwrap().into_any().unbind(),
            ];
            let key = PyTuple::new(py, &key_items).expect("key tuple");
            let removed = tree.delete(key.into_any()).expect("delete works");
            assert!(removed);
            let hits = tree.search(12, 18, true, py).expect("search after delete");
            assert!(hits.iter().all(|(l, r, _)| (*l, *r) != (12, 18)));
        });
    }
}
