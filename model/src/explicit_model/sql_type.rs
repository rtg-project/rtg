// See rust-postgres
use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// See https://github.com/sfackler/rust-postgres/blob/fc10985f9fdf0903893109bc951fb5891539bf97/postgres-types/src/type_gen.rs
#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema, Clone, Display)]
#[serde(rename_all = "camelCase")]
pub enum Type {
  #[display(fmt = "bool")]
  Bool,
  #[display(fmt = "bytea")]
  Bytea,
  #[display(fmt = "char")]
  Char,
  #[display(fmt = "name")]
  Name,
  #[display(fmt = "int8")]
  Int8,
  #[display(fmt = "int2")]
  Int2,
  #[display(fmt = "int2vector")]
  Int2Vector,
  #[display(fmt = "int4")]
  Int4,
  #[display(fmt = "regproc")]
  Regproc,
  #[display(fmt = "text")]
  Text,
  #[display(fmt = "oid")]
  Oid,
  #[display(fmt = "tid")]
  Tid,
  #[display(fmt = "xid")]
  Xid,
  #[display(fmt = "cid")]
  Cid,
  #[display(fmt = "oidvector")]
  OidVector,
  #[display(fmt = "pgddlcommand")]
  PgDdlCommand,
  #[display(fmt = "json")]
  Json,
  #[display(fmt = "xml")]
  Xml,
  #[display(fmt = "xml[]")]
  XmlArray,
  #[display(fmt = "pgnodetree")]
  PgNodeTree,
  #[display(fmt = "json[]")]
  JsonArray,
  #[display(fmt = "tableamhandler")]
  TableAmHandler,
  #[display(fmt = "xid8[]")]
  Xid8Array,
  #[display(fmt = "indexamhandler")]
  IndexAmHandler,
  #[display(fmt = "point")]
  Point,
  #[display(fmt = "lseg")]
  Lseg,
  #[display(fmt = "path")]
  Path,
  #[display(fmt = "box")]
  Box,
  #[display(fmt = "polygon")]
  Polygon,
  #[display(fmt = "line")]
  Line,
  #[display(fmt = "line[]")]
  LineArray,
  #[display(fmt = "cidr")]
  Cidr,
  #[display(fmt = "cidr[]")]
  CidrArray,
  #[display(fmt = "float4")]
  Float4,
  #[display(fmt = "float8")]
  Float8,
  #[display(fmt = "unknown")]
  Unknown,
  #[display(fmt = "circle")]
  Circle,
  #[display(fmt = "circle[]")]
  CircleArray,
  #[display(fmt = "macaddr8")]
  Macaddr8,
  #[display(fmt = "macaddr8[]")]
  Macaddr8Array,
  #[display(fmt = "money")]
  Money,
  #[display(fmt = "money[]")]
  MoneyArray,
  #[display(fmt = "macaddr")]
  Macaddr,
  #[display(fmt = "inet")]
  Inet,
  #[display(fmt = "bool[]")]
  BoolArray,
  #[display(fmt = "bytea[]")]
  ByteaArray,
  #[display(fmt = "char[]")]
  CharArray,
  #[display(fmt = "name[]")]
  NameArray,
  #[display(fmt = "int2[]")]
  Int2Array,
  #[display(fmt = "int2vector[]")]
  Int2VectorArray,
  #[display(fmt = "int4[]")]
  Int4Array,
  #[display(fmt = "regproc[]")]
  RegprocArray,
  #[display(fmt = "text[]")]
  TextArray,
  #[display(fmt = "tid[]")]
  TidArray,
  #[display(fmt = "xid[]")]
  XidArray,
  #[display(fmt = "cid[]")]
  CidArray,
  #[display(fmt = "oidvector[]")]
  OidVectorArray,
  #[display(fmt = "bpchar[]")]
  BpcharArray,
  #[display(fmt = "varchar[]")]
  VarcharArray,
  #[display(fmt = "int8[]")]
  Int8Array,
  #[display(fmt = "point[]")]
  PointArray,
  #[display(fmt = "lseg[]")]
  LsegArray,
  #[display(fmt = "path[]")]
  PathArray,
  #[display(fmt = "box[]")]
  BoxArray,
  #[display(fmt = "float4[]")]
  Float4Array,
  #[display(fmt = "float8[]")]
  Float8Array,
  #[display(fmt = "polygon[]")]
  PolygonArray,
  #[display(fmt = "oid[]")]
  OidArray,
  #[display(fmt = "aclitem")]
  Aclitem,
  #[display(fmt = "aclitem[]")]
  AclitemArray,
  #[display(fmt = "macaddr[]")]
  MacaddrArray,
  #[display(fmt = "inet[]")]
  InetArray,
  #[display(fmt = "bpchar")]
  Bpchar,
  #[display(fmt = "varchar")]
  Varchar,
  #[display(fmt = "date")]
  Date,
  #[display(fmt = "time")]
  Time,
  #[display(fmt = "timestamp")]
  Timestamp,
  #[display(fmt = "timestamp[]")]
  TimestampArray,
  #[display(fmt = "date[]")]
  DateArray,
  #[display(fmt = "time[]")]
  TimeArray,
  #[display(fmt = "timestamptz")]
  Timestamptz,
  #[display(fmt = "timestamptz[]")]
  TimestamptzArray,
  #[display(fmt = "interval")]
  Interval,
  #[display(fmt = "interval[]")]
  IntervalArray,
  #[display(fmt = "numeric[]")]
  NumericArray,
  #[display(fmt = "cstring[]")]
  CstringArray,
  #[display(fmt = "timetz")]
  Timetz,
  #[display(fmt = "timetz[]")]
  TimetzArray,
  #[display(fmt = "bit")]
  Bit,
  #[display(fmt = "bit[]")]
  BitArray,
  #[display(fmt = "varbit")]
  Varbit,
  #[display(fmt = "varbit[]")]
  VarbitArray,
  #[display(fmt = "numeric")]
  Numeric,
  #[display(fmt = "refcursor")]
  Refcursor,
  #[display(fmt = "refcursor[]")]
  RefcursorArray,
  #[display(fmt = "regprocedure")]
  Regprocedure,
  #[display(fmt = "regoper")]
  Regoper,
  #[display(fmt = "regoperator")]
  Regoperator,
  #[display(fmt = "regclass")]
  Regclass,
  #[display(fmt = "regtype")]
  Regtype,
  #[display(fmt = "regprocedure[]")]
  RegprocedureArray,
  #[display(fmt = "regoper[]")]
  RegoperArray,
  #[display(fmt = "regoperator[]")]
  RegoperatorArray,
  #[display(fmt = "regclass[]")]
  RegclassArray,
  #[display(fmt = "regtype[]")]
  RegtypeArray,
  #[display(fmt = "record")]
  Record,
  #[display(fmt = "cstring")]
  Cstring,
  #[display(fmt = "any")]
  Any,
  #[display(fmt = "anyarray")]
  Anyarray,
  #[display(fmt = "void")]
  Void,
  #[display(fmt = "trigger")]
  Trigger,
  #[display(fmt = "languagehandler")]
  LanguageHandler,
  #[display(fmt = "internal")]
  Internal,
  #[display(fmt = "anyelement")]
  Anyelement,
  #[display(fmt = "record[]")]
  RecordArray,
  #[display(fmt = "anynonarray")]
  Anynonarray,
  #[display(fmt = "txidsnapshot[]")]
  TxidSnapshotArray,
  #[display(fmt = "uuid")]
  Uuid,
  #[display(fmt = "uuid[]")]
  UuidArray,
  #[display(fmt = "txidsnapshot")]
  TxidSnapshot,
  #[display(fmt = "fdwhandler")]
  FdwHandler,
  #[display(fmt = "pglsn")]
  PgLsn,
  #[display(fmt = "pglsn[]")]
  PgLsnArray,
  #[display(fmt = "tsmhandler")]
  TsmHandler,
  #[display(fmt = "pgndistinct")]
  PgNdistinct,
  #[display(fmt = "pgdependencies")]
  PgDependencies,
  #[display(fmt = "anyenum")]
  Anyenum,
  #[display(fmt = "tsvector")]
  TsVector,
  #[display(fmt = "tsquery")]
  Tsquery,
  #[display(fmt = "gtsvector")]
  GtsVector,
  #[display(fmt = "tsvector[]")]
  TsVectorArray,
  #[display(fmt = "gtsvector[]")]
  GtsVectorArray,
  #[display(fmt = "tsquery[]")]
  TsqueryArray,
  #[display(fmt = "regconfig")]
  Regconfig,
  #[display(fmt = "regconfig[]")]
  RegconfigArray,
  #[display(fmt = "regdictionary")]
  Regdictionary,
  #[display(fmt = "regdictionary[]")]
  RegdictionaryArray,
  #[display(fmt = "jsonb")]
  Jsonb,
  #[display(fmt = "jsonb[]")]
  JsonbArray,
  #[display(fmt = "anyrange")]
  AnyRange,
  #[display(fmt = "eventtrigger")]
  EventTrigger,
  #[display(fmt = "int4range")]
  Int4Range,
  #[display(fmt = "int4range[]")]
  Int4RangeArray,
  #[display(fmt = "numrange")]
  NumRange,
  #[display(fmt = "numrange[]")]
  NumRangeArray,
  #[display(fmt = "tsrange")]
  TsRange,
  #[display(fmt = "tsrange[]")]
  TsRangeArray,
  #[display(fmt = "tstzrange")]
  TstzRange,
  #[display(fmt = "tstzrange[]")]
  TstzRangeArray,
  #[display(fmt = "daterange")]
  DateRange,
  #[display(fmt = "daterange[]")]
  DateRangeArray,
  #[display(fmt = "int8range")]
  Int8Range,
  #[display(fmt = "int8range[]")]
  Int8RangeArray,
  #[display(fmt = "jsonpath")]
  Jsonpath,
  #[display(fmt = "jsonpath[]")]
  JsonpathArray,
  #[display(fmt = "regnamespace")]
  Regnamespace,
  #[display(fmt = "regnamespace[]")]
  RegnamespaceArray,
  #[display(fmt = "regrole")]
  Regrole,
  #[display(fmt = "regrole[]")]
  RegroleArray,
  #[display(fmt = "regcollation")]
  Regcollation,
  #[display(fmt = "regcollation[]")]
  RegcollationArray,
  #[display(fmt = "pgmcvlist")]
  PgMcvList,
  #[display(fmt = "pgsnapshot")]
  PgSnapshot,
  #[display(fmt = "pgsnapshot[]")]
  PgSnapshotArray,
  #[display(fmt = "xid8")]
  Xid8,
  #[display(fmt = "anycompatible")]
  Anycompatible,
  #[display(fmt = "anycompatiblearray")]
  Anycompatiblearray,
  #[display(fmt = "anycompatiblenonarray")]
  Anycompatiblenonarray,
  #[display(fmt = "anycompatiblerange")]
  AnycompatibleRange,
  #[serde(rename_all = "camelCase")]
  #[display(fmt = "{}", "_0")]
  Other(String),
}

#[cfg(test)]
mod tests {
  use super::*;
  use similar_asserts::assert_eq;

  #[test]
  fn serialize1() {
    let value = Type::Bit;

    match serde_json::to_string_pretty(&value) {
      Ok(string) => {
        assert_eq!(string, r#""bit""#);
      }
      Err(e) => panic!("{}", e),
    }
  }

  #[test]
  fn serialize2() {
    let value = Type::Int8;

    match serde_json::to_string_pretty(&value) {
      Ok(string) => {
        assert_eq!(string, r#""int8""#);
      }
      Err(e) => panic!("{}", e),
    }
  }

  #[test]
  fn serialize_other() {
    let value = Type::Other("wow".to_owned());

    match serde_json::to_string_pretty(&value) {
      Ok(string) => {
        assert_eq!(
          string,
          r#"{
  "other": "wow"
}"#
        );
      }
      Err(e) => panic!("{}", e),
    }
  }
}
