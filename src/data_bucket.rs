use super::Source;
use futures::stream;
use futures::Stream;
use std::collections::HashMap;

/// LinkType
/// An enum for each type of relationship between two DataBlobs
pub enum LinkType {
    /// Each primary unit of the data corresponds to one unit of the linked data
    OneToOne,
    /// The linker data represents handles to the linked data
    Handle,
    /// The linker data is a reduction of the linked data
    Reduced,
}

/// Link
/// Structure defining the relationship between two DataBlobs
pub struct Link {
    /// nature of the link between the data
    pub nature: LinkType,
    /// the name of the data that is referencing some other data
    pub linker: String,
    /// the name of the primary data being referenced
    pub linkee: String,
}

/// MetaData
/// A structure describing the data of a DataBlob
pub struct MetaData {
    /// name of the data array
    pub name: String,
    /// units of the data
    pub units: Option<String>,
    /// general description of the data
    pub description: Option<String>,
    /// full dimensions of the data
    pub dimensions: Vec<usize>,
    /// dimensions of a unit size of the data
    pub unitary_dimensions: Vec<usize>,
    /// links to other data in the same bucket
    pub links: Vec<Link>,
}

/// DataBlob
/// A structure holding an array of data with its inherent meta-data
pub struct DataBlob<T> {
    data: Vec<T>,
    meta: MetaData,
}

impl<T> DataBlob<T> {
    /// constructor
    pub fn new(new_data: Vec<T>, new_meta: MetaData) -> Self {
        Self {
            data: new_data,
            meta: new_meta,
        }
    }
    /// get the underlying data immutably
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }
    /// get the underlying data mutably
    pub fn get_mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }
    /// get the associated meta data immutably
    pub fn get_meta_data(&self) -> &MetaData {
        &self.meta
    }
    /// get the associated meta data mutably
    pub fn get_mut_meta_data(&mut self) -> &mut MetaData {
        &mut self.meta
    }
}

impl<T: Clone + 'static> Source<T> for DataBlob<T> {
    fn stream(&self) -> Box<dyn Stream<Item = T>> {
        Box::new(stream::iter(self.data.clone()))
    }
}

/// DataBucketBlobs
/// An enum wrapping for all the different primitive typed DataBlobs
pub enum DataBucketBlob {
    Bool(DataBlob<bool>),
    Char(DataBlob<char>),
    Int8(DataBlob<i8>),
    U8(DataBlob<u8>),
    Int16(DataBlob<i16>),
    U16(DataBlob<u16>),
    Int32(DataBlob<i32>),
    U32(DataBlob<u32>),
    Int64(DataBlob<i64>),
    U64(DataBlob<u64>),
    Int128(DataBlob<i128>),
    U128(DataBlob<u128>),
    ISize(DataBlob<isize>),
    USize(DataBlob<usize>),
    Float32(DataBlob<f32>),
    Float64(DataBlob<f64>),
    Str(DataBlob<String>),
}

macro_rules! meta_data_unwrap {
  ($($x:ident),*) => {
    pub fn get_meta_data(&self) -> &MetaData {
      match *self {
        $( DataBucketBlob::$x(ref blob) => blob.get_meta_data(), )*
      }
    }
  }
}

macro_rules! mut_meta_data_unwrap {
  ($($x:ident),*) => {
    pub fn get_mut_meta_data(&mut self) -> &mut MetaData {
      match self {
        $( DataBucketBlob::$x(blob) => blob.get_mut_meta_data(), )*
      }
    }
  }
}

impl DataBucketBlob {
    meta_data_unwrap!(
        Bool, Char, Int8, U8, Int16, U16, Int32, U32, Int64, U64, Int128, U128, ISize, USize,
        Float32, Float64, Str
    );
    mut_meta_data_unwrap!(
        Bool, Char, Int8, U8, Int16, U16, Int32, U32, Int64, U64, Int128, U128, ISize, USize,
        Float32, Float64, Str
    );
}

/// DataBucket
/// A flexible structure for holding heterogeneous data
pub struct DataBucket {
    data: HashMap<String, DataBucketBlob>,
}

impl DataBucket {
    /// empty constructor
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    /// get a data blob
    pub fn get_blob(&self, blob_name: &String) -> Option<&DataBucketBlob> {
        self.data.get(blob_name)
    }
    /// add a blob
    pub fn add_blob(&mut self, new_blob: DataBucketBlob) -> Option<DataBucketBlob> {
        let name = &new_blob.get_meta_data().name;
        if self.data.contains_key(name) {
            return Some(new_blob);
        }
        self.data.insert(name.clone(), new_blob);
        None
    }
    // remove a blob
    pub fn pop_blob(&mut self, name: String) -> Option<DataBucketBlob> {
      self.data.remove(&name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::zip;

    fn make_int_blob() -> DataBlob<i8> {
        let meta = MetaData {
            name: "Test data".to_string(),
            description: None,
            units: None,
            unitary_dimensions: vec![1],
            dimensions: vec![10],
            links: Vec::new(),
        };
        let data: Vec<i8> = (0..10).collect();
        DataBlob::new(data, meta)
    }

    #[test]
    fn test_int_data_blob() {
        let blob = make_int_blob();
        for (idx, val) in zip(0..10, blob.get_data().iter()) {
            assert_eq!(idx, *val, "Failure to get data from blob");
        }
    }

    #[test]
    fn test_double_data_blob() {
        let meta = MetaData {
            name: "Test data".to_string(),
            description: None,
            units: None,
            unitary_dimensions: vec![1],
            dimensions: vec![10],
            links: Vec::new(),
        };
        let data: Vec<i8> = (0..10).collect();
        let data: Vec<f64> = data.into_iter().map(|x| f64::from(x) / 10.0_f64).collect();
        let blob = DataBlob::new(data, meta);
        for (idx, val) in zip(0..10, blob.get_data().iter()) {
            assert_eq!(idx as f64 / 10.0, *val, "Failure to get data from blob");
        }
    }

    #[test]
    fn test_add_data_bucket() {
        let mut bucket = DataBucket::new();
        bucket.add_blob(DataBucketBlob::Int8(make_int_blob()));
        let blob = bucket.get_blob(&"Test data".to_string());
        let blob = match blob.unwrap() {
            DataBucketBlob::Int8(ref b) => b,
            _ => panic!("Could not match blob"),
        };
        for (idx, val) in zip(0..10, blob.get_data().iter()) {
            assert_eq!(idx, *val, "Failure to retreive data from bucket");
        }
    }

    #[test]
    fn test_pop_data_bucket() {
        let mut bucket = DataBucket::new();
        bucket.add_blob(DataBucketBlob::Int8(make_int_blob()));
        let blob = bucket.pop_blob("Test data".to_string());
        assert!(!blob.is_none(), "Popped back None from data bucket");
        let blob = blob.unwrap();
        let blob = match blob {
            DataBucketBlob::Int8(b) => b,
            _ => panic!("Could not match blob"),
        };
        for (idx, val) in zip(0..10, blob.get_data().iter()) {
            assert_eq!(idx, *val, "Failure to pop data from bucket");
        }
    }
}
