use std::borrow::Cow;
use std::fmt;
use std::cmp::Ordering;
use std::io::{stdin,stdout,Write};

#[derive(Debug)]
pub enum PyObject {
    String { value: Cow<'static, str> },
	Integer { value: i64 },
	Float {value: f64},
}

impl PyObject {
	pub fn get_int(&self) -> Option<i64> {
		match self{
			PyObject::Integer{value} => Some(*value),
			_=> None,
		}
	}
	
	pub fn to_int(&self) -> Option<PyObject> {
		match self {
			PyObject::String{value} => Some(PyObject::from(value.parse::<i64>().unwrap())),
			PyObject::Integer{value} => Some(PyObject::from(*value)),
			_=> unimplemented!("int({:?})", self)
		}
	}
	
	pub fn to_float(&self) -> Option<PyObject> {
		match self {
			PyObject::String{value} => Some(PyObject::from(value.parse::<f64>().unwrap())),
			PyObject::Float{value} => Some(PyObject::from(*value)),
			_=> unimplemented!("float({:?})", self)
		}
	}

  pub fn mul<T: AsRef<PyObject>>(&self, obj: T) -> PyObject {
	let obj = obj.as_ref();
	match (self, obj) {
		(PyObject::Float{value: value1}, PyObject::Float{value: value2}) => PyObject::from(value1 * value2),
		(PyObject::Float{value: value1}, PyObject::Integer{value: value2}) => PyObject::from(value1 * *value2 as f64),
		(PyObject::Integer{value: value1}, PyObject::Integer{value: value2}) => PyObject::from(value1 * value2),
		_ => unimplemented!("{:?} * {:?}", self, obj),
	}
  }
  
  pub fn add<T: AsRef<PyObject>>(&self, obj: T) -> PyObject {
	let obj = obj.as_ref();
    match (self, obj) {
		(PyObject::Float{value: value1}, PyObject::Float{value: value2}) => PyObject::from(value1 + value2),
		(PyObject::Float{value: value1}, PyObject::Integer{value: value2}) => PyObject::from(value1 + *value2 as f64),
		_ => unimplemented!("{:?} + {:?}", self, obj),
	}
  }
  
  pub fn sub<T: AsRef<PyObject>>(&self, obj: T) -> PyObject {
	let obj = obj.as_ref();
    match (self, obj) {
		(PyObject::Float{value: value1}, PyObject::Float{value: value2}) => PyObject::from(value1 - value2),
		(PyObject::Float{value: value1}, PyObject::Integer{value: value2}) => PyObject::from(value1 - *value2 as f64),
		(PyObject::Integer{value: value1}, PyObject::Integer{value: value2}) => PyObject::from(value1 - value2),
		_ => unimplemented!("{:?} - {:?}", self, obj),
	}
  }
  
  pub fn div<T: AsRef<PyObject>>(&self, obj: T) -> PyObject {
	let obj = obj.as_ref();
	match (self, obj) {
		(PyObject::Float{value: value1}, PyObject::Float{value: value2}) => PyObject::from(value1 / value2),
		(PyObject::Float{value: value1}, PyObject::Integer{value: value2}) => PyObject::from(value1 / *value2 as f64),
		_ => unimplemented!("{:?} + {:?}", self, obj),
	}
  }
	
	pub fn pow(&self, obj: &PyObject) -> PyObject {
		let obj = obj.as_ref();
		match (self, obj) {
			(PyObject::Float{value: value1}, PyObject::Float{value: value2}) => PyObject::from(value1.powf(*value2)),
			(PyObject::Float{value: value1}, PyObject::Integer{value: value2}) => PyObject::from(value1.powf(*value2 as f64)),
			_ => unimplemented!("{:?} pow {:?}", self, obj),
		}
	}
}

impl<'a> AsRef<PyObject> for PyObject{
    fn as_ref(&self) -> &PyObject {
		self
	}
}

impl fmt::Display for PyObject {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            PyObject::String { value } => {
                write!(fmt, "{}", value)?;
            },
			PyObject::Integer {value } => {
				write!(fmt, "{}", value)?;
			},
			PyObject::Float {value } => {
				write!(fmt, "{}", value)?;
			}
        }

        Ok(())
    }
}

impl PartialEq<PyObject> for PyObject {
	fn eq(&self, other: &Self) -> bool {
        match (self, other) {
			(PyObject::Integer { value: value1}, PyObject::Integer {value: value2}) => value1 == value2,
			(PyObject::String { value: value1}, PyObject::String {value: value2}) => value1 == value2,
			_=> unimplemented!("{:?} == {:?}", self, other),			
		}
    }
}

impl PartialOrd for PyObject {
    fn partial_cmp(&self, other: &PyObject) -> Option<Ordering> {
		match (self, other) {
			(PyObject::String { value: value1}, PyObject::String {value: value2}) => value1.partial_cmp(value2),
			_=> unimplemented!("ord {:?}, {:?}", self, other),
		}
    }
}

impl From<&'static str> for PyObject {
	fn from(data: &'static str) -> Self {
		PyObject::String {
			value: data.into()
		}
	}
}

impl From<String> for PyObject {
	fn from(data: String) -> Self {
		PyObject::String {
			value: data.into()
		}
	}
}

impl From<i64> for PyObject {
	fn from(data: i64) -> Self {
		PyObject::Integer {
			value: data
		}
	}
}

impl From<f64> for PyObject {
	fn from(data: f64) -> Self {
		PyObject::Float {
			value: data
		}
	}
}

pub fn print(args: &[&PyObject]){
	for arg in args {
		print!("{}", arg);
	}
	print!("\n");
}

pub fn input(args: &[&PyObject]) -> PyObject {
	print!("{}", args[0]);
	let _=stdout().flush();
	let mut s=String::new();
	stdin().read_line(&mut s).expect("Did not enter a correct string");
	if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
	PyObject::from(s)
}

pub fn float(args: &[&PyObject]) -> PyObject {
	args[0].to_float().unwrap()
}

pub fn int(args: &[&PyObject]) -> PyObject {
	args[0].to_int().unwrap()
}

