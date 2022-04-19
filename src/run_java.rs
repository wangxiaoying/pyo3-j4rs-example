use j4rs::{ClasspathEntry, InvocationArg, Jvm, JvmBuilder};
use std::convert::TryFrom;
use std::fs;


pub fn run_java(in_str: &str) {
    let path = fs::canonicalize("./pyo3-test.jar").unwrap();
    let entry = ClasspathEntry::new(path.to_str().unwrap());
    let jvm: Jvm = JvmBuilder::new().classpath_entry(entry).build().unwrap();

    let str = InvocationArg::try_from(in_str).unwrap();
    let test = jvm.create_instance("pyo3.test.Pyo3Test", &[]).unwrap();
    jvm.invoke(&test, "run", &[str]).unwrap();
}