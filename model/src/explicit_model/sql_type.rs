// See rust-postgres
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// See https://github.com/sfackler/rust-postgres/blob/fc10985f9fdf0903893109bc951fb5891539bf97/postgres-types/src/type_gen.rs
#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Type {
  Bool,
  Bytea,
  Char,
  Name,
  Int8,
  Int2,
  Int2Vector,
  Int4,
  Regproc,
  Text,
  Oid,
  Tid,
  Xid,
  Cid,
  OidVector,
  PgDdlCommand,
  Json,
  Xml,
  XmlArray,
  PgNodeTree,
  JsonArray,
  TableAmHandler,
  Xid8Array,
  IndexAmHandler,
  Point,
  Lseg,
  Path,
  Box,
  Polygon,
  Line,
  LineArray,
  Cidr,
  CidrArray,
  Float4,
  Float8,
  Unknown,
  Circle,
  CircleArray,
  Macaddr8,
  Macaddr8Array,
  Money,
  MoneyArray,
  Macaddr,
  Inet,
  BoolArray,
  ByteaArray,
  CharArray,
  NameArray,
  Int2Array,
  Int2VectorArray,
  Int4Array,
  RegprocArray,
  TextArray,
  TidArray,
  XidArray,
  CidArray,
  OidVectorArray,
  BpcharArray,
  VarcharArray,
  Int8Array,
  PointArray,
  LsegArray,
  PathArray,
  BoxArray,
  Float4Array,
  Float8Array,
  PolygonArray,
  OidArray,
  Aclitem,
  AclitemArray,
  MacaddrArray,
  InetArray,
  Bpchar,
  Varchar,
  Date,
  Time,
  Timestamp,
  TimestampArray,
  DateArray,
  TimeArray,
  Timestamptz,
  TimestamptzArray,
  Interval,
  IntervalArray,
  NumericArray,
  CstringArray,
  Timetz,
  TimetzArray,
  Bit,
  BitArray,
  Varbit,
  VarbitArray,
  Numeric,
  Refcursor,
  RefcursorArray,
  Regprocedure,
  Regoper,
  Regoperator,
  Regclass,
  Regtype,
  RegprocedureArray,
  RegoperArray,
  RegoperatorArray,
  RegclassArray,
  RegtypeArray,
  Record,
  Cstring,
  Any,
  Anyarray,
  Void,
  Trigger,
  LanguageHandler,
  Internal,
  Anyelement,
  RecordArray,
  Anynonarray,
  TxidSnapshotArray,
  Uuid,
  UuidArray,
  TxidSnapshot,
  FdwHandler,
  PgLsn,
  PgLsnArray,
  TsmHandler,
  PgNdistinct,
  PgDependencies,
  Anyenum,
  TsVector,
  Tsquery,
  GtsVector,
  TsVectorArray,
  GtsVectorArray,
  TsqueryArray,
  Regconfig,
  RegconfigArray,
  Regdictionary,
  RegdictionaryArray,
  Jsonb,
  JsonbArray,
  AnyRange,
  EventTrigger,
  Int4Range,
  Int4RangeArray,
  NumRange,
  NumRangeArray,
  TsRange,
  TsRangeArray,
  TstzRange,
  TstzRangeArray,
  DateRange,
  DateRangeArray,
  Int8Range,
  Int8RangeArray,
  Jsonpath,
  JsonpathArray,
  Regnamespace,
  RegnamespaceArray,
  Regrole,
  RegroleArray,
  Regcollation,
  RegcollationArray,
  PgMcvList,
  PgSnapshot,
  PgSnapshotArray,
  Xid8,
  Anycompatible,
  Anycompatiblearray,
  Anycompatiblenonarray,
  AnycompatibleRange,
  #[serde(rename_all = "camelCase")]
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
    let value = Type::Other("wow".to_string());

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
