use mongodb::error::Error;

pub const NOT_FOUND : Error = Error::CursorNotFoundError;

#[macro_export]
macro_rules! find {
    ($db: expr, $name: expr, $query: expr) => {{
        find!($db, $name, $query, None)
    }};
    ($db: expr, $name: expr, $query: expr, $options: expr) => {{
        let coll = $db.collection($name);
        coll.find($query, $options)
    }};
}

#[macro_export]
macro_rules! extract_object_id {
    ($res: expr, $name: expr) => {{
        $res.get_object_id($name).map(|val| val.to_owned())
    }}
}

#[macro_export]
macro_rules! extract_string {
    ($res: expr, $name: expr) => {{
        $res.get_str($name).map(|val| val.to_string().to_owned())
    }}
}

#[macro_export]
macro_rules! extract_date {
    ($res: expr, $name: expr) => {{
        $res.get_utc_datetime($name).map(|val| val.to_owned())
    }}
}

#[macro_export]
macro_rules! fold {
    ($cursor: expr, $result: expr, $f: expr) => {{
        for doc in $cursor {
            let x = $f(doc);
            if x.is_some() {
                $result.push(x.unwrap());
            }
        }
        $result
    }}
}

#[macro_use]
pub mod event;