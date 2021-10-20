// See rust-postgres
use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// See https://github.com/sfackler/rust-postgres/blob/fc10985f9fdf0903893109bc951fb5891539bf97/postgres-types/src/type_gen.rs
#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema, Clone, Display)]
#[serde(rename_all = "camelCase")]
pub enum Type {
  #[display(fmt = "Bool")]
  Bool,
  #[display(fmt = "Bytea")]
  Bytea,
  #[display(fmt = "Char")]
  Char,
  #[display(fmt = "Name")]
  Name,
  #[display(fmt = "Int8")]
  Int8,
  #[display(fmt = "Int2")]
  Int2,
  #[display(fmt = "Int2Vector")]
  Int2Vector,
  #[display(fmt = "Int4")]
  Int4,
  #[display(fmt = "Regproc")]
  Regproc,
  #[display(fmt = "Text")]
  Text,
  #[display(fmt = "Oid")]
  Oid,
  #[display(fmt = "Tid")]
  Tid,
  #[display(fmt = "Xid")]
  Xid,
  #[display(fmt = "Cid")]
  Cid,
  #[display(fmt = "OidVector")]
  OidVector,
  #[display(fmt = "PgDdlCommand")]
  PgDdlCommand,
  #[display(fmt = "Json")]
  Json,
  #[display(fmt = "Xml")]
  Xml,
  #[display(fmt = "Xml[]")]
  XmlArray,
  #[display(fmt = "PgNodeTree")]
  PgNodeTree,
  #[display(fmt = "Json[]")]
  JsonArray,
  #[display(fmt = "TableAmHandler")]
  TableAmHandler,
  #[display(fmt = "Xid8[]")]
  Xid8Array,
  #[display(fmt = "IndexAmHandler")]
  IndexAmHandler,
  #[display(fmt = "Point")]
  Point,
  #[display(fmt = "Lseg")]
  Lseg,
  #[display(fmt = "Path")]
  Path,
  #[display(fmt = "Box")]
  Box,
  #[display(fmt = "Polygon")]
  Polygon,
  #[display(fmt = "Line")]
  Line,
  #[display(fmt = "Line[]")]
  LineArray,
  #[display(fmt = "Cidr")]
  Cidr,
  #[display(fmt = "Cidr[]")]
  CidrArray,
  #[display(fmt = "Float4")]
  Float4,
  #[display(fmt = "Float8")]
  Float8,
  #[display(fmt = "Unknown")]
  Unknown,
  #[display(fmt = "Circle")]
  Circle,
  #[display(fmt = "Circle[]")]
  CircleArray,
  #[display(fmt = "Macaddr8")]
  Macaddr8,
  #[display(fmt = "Macaddr8[]")]
  Macaddr8Array,
  #[display(fmt = "Money")]
  Money,
  #[display(fmt = "Money[]")]
  MoneyArray,
  #[display(fmt = "Macaddr")]
  Macaddr,
  #[display(fmt = "Inet")]
  Inet,
  #[display(fmt = "Bool[]")]
  BoolArray,
  #[display(fmt = "Bytea[]")]
  ByteaArray,
  #[display(fmt = "Char[]")]
  CharArray,
  #[display(fmt = "Name[]")]
  NameArray,
  #[display(fmt = "Int2[]")]
  Int2Array,
  #[display(fmt = "Int2Vector[]")]
  Int2VectorArray,
  #[display(fmt = "Int4[]")]
  Int4Array,
  #[display(fmt = "Regproc[]")]
  RegprocArray,
  #[display(fmt = "Text[]")]
  TextArray,
  #[display(fmt = "Tid[]")]
  TidArray,
  #[display(fmt = "Xid[]")]
  XidArray,
  #[display(fmt = "Cid[]")]
  CidArray,
  #[display(fmt = "OidVector[]")]
  OidVectorArray,
  #[display(fmt = "Bpchar[]")]
  BpcharArray,
  #[display(fmt = "Varchar[]")]
  VarcharArray,
  #[display(fmt = "Int8[]")]
  Int8Array,
  #[display(fmt = "Point[]")]
  PointArray,
  #[display(fmt = "Lseg[]")]
  LsegArray,
  #[display(fmt = "Path[]")]
  PathArray,
  #[display(fmt = "Box[]")]
  BoxArray,
  #[display(fmt = "Float4[]")]
  Float4Array,
  #[display(fmt = "Float8[]")]
  Float8Array,
  #[display(fmt = "Polygon[]")]
  PolygonArray,
  #[display(fmt = "Oid[]")]
  OidArray,
  #[display(fmt = "Aclitem")]
  Aclitem,
  #[display(fmt = "Aclitem[]")]
  AclitemArray,
  #[display(fmt = "Macaddr[]")]
  MacaddrArray,
  #[display(fmt = "Inet[]")]
  InetArray,
  #[display(fmt = "Bpchar")]
  Bpchar,
  #[display(fmt = "Varchar")]
  Varchar,
  #[display(fmt = "Date")]
  Date,
  #[display(fmt = "Time")]
  Time,
  #[display(fmt = "Timestamp")]
  Timestamp,
  #[display(fmt = "Timestamp[]")]
  TimestampArray,
  #[display(fmt = "Date[]")]
  DateArray,
  #[display(fmt = "Time[]")]
  TimeArray,
  #[display(fmt = "Timestamptz")]
  Timestamptz,
  #[display(fmt = "Timestamptz[]")]
  TimestamptzArray,
  #[display(fmt = "Interval")]
  Interval,
  #[display(fmt = "Interval[]")]
  IntervalArray,
  #[display(fmt = "Numeric[]")]
  NumericArray,
  #[display(fmt = "Cstring[]")]
  CstringArray,
  #[display(fmt = "Timetz")]
  Timetz,
  #[display(fmt = "Timetz[]")]
  TimetzArray,
  #[display(fmt = "Bit")]
  Bit,
  #[display(fmt = "Bit[]")]
  BitArray,
  #[display(fmt = "Varbit")]
  Varbit,
  #[display(fmt = "Varbit[]")]
  VarbitArray,
  #[display(fmt = "Numeric")]
  Numeric,
  #[display(fmt = "Refcursor")]
  Refcursor,
  #[display(fmt = "Refcursor[]")]
  RefcursorArray,
  #[display(fmt = "Regprocedure")]
  Regprocedure,
  #[display(fmt = "Regoper")]
  Regoper,
  #[display(fmt = "Regoperator")]
  Regoperator,
  #[display(fmt = "Regclass")]
  Regclass,
  #[display(fmt = "Regtype")]
  Regtype,
  #[display(fmt = "Regprocedure[]")]
  RegprocedureArray,
  #[display(fmt = "Regoper[]")]
  RegoperArray,
  #[display(fmt = "Regoperator[]")]
  RegoperatorArray,
  #[display(fmt = "Regclass[]")]
  RegclassArray,
  #[display(fmt = "Regtype[]")]
  RegtypeArray,
  #[display(fmt = "Record")]
  Record,
  #[display(fmt = "Cstring")]
  Cstring,
  #[display(fmt = "Any")]
  Any,
  #[display(fmt = "Anyarray")]
  Anyarray,
  #[display(fmt = "Void")]
  Void,
  #[display(fmt = "Trigger")]
  Trigger,
  #[display(fmt = "LanguageHandler")]
  LanguageHandler,
  #[display(fmt = "Internal")]
  Internal,
  #[display(fmt = "Anyelement")]
  Anyelement,
  #[display(fmt = "Record[]")]
  RecordArray,
  #[display(fmt = "Anynonarray")]
  Anynonarray,
  #[display(fmt = "TxidSnapshot[]")]
  TxidSnapshotArray,
  #[display(fmt = "Uuid")]
  Uuid,
  #[display(fmt = "Uuid[]")]
  UuidArray,
  #[display(fmt = "TxidSnapshot")]
  TxidSnapshot,
  #[display(fmt = "FdwHandler")]
  FdwHandler,
  #[display(fmt = "PgLsn")]
  PgLsn,
  #[display(fmt = "PgLsn[]")]
  PgLsnArray,
  #[display(fmt = "TsmHandler")]
  TsmHandler,
  #[display(fmt = "PgNdistinct")]
  PgNdistinct,
  #[display(fmt = "PgDependencies")]
  PgDependencies,
  #[display(fmt = "Anyenum")]
  Anyenum,
  #[display(fmt = "TsVector")]
  TsVector,
  #[display(fmt = "Tsquery")]
  Tsquery,
  #[display(fmt = "GtsVector")]
  GtsVector,
  #[display(fmt = "TsVector[]")]
  TsVectorArray,
  #[display(fmt = "GtsVector[]")]
  GtsVectorArray,
  #[display(fmt = "Tsquery[]")]
  TsqueryArray,
  #[display(fmt = "Regconfig")]
  Regconfig,
  #[display(fmt = "Regconfig[]")]
  RegconfigArray,
  #[display(fmt = "Regdictionary")]
  Regdictionary,
  #[display(fmt = "Regdictionary[]")]
  RegdictionaryArray,
  #[display(fmt = "Jsonb")]
  Jsonb,
  #[display(fmt = "Jsonb[]")]
  JsonbArray,
  #[display(fmt = "AnyRange")]
  AnyRange,
  #[display(fmt = "EventTrigger")]
  EventTrigger,
  #[display(fmt = "Int4Range")]
  Int4Range,
  #[display(fmt = "Int4Range[]")]
  Int4RangeArray,
  #[display(fmt = "NumRange")]
  NumRange,
  #[display(fmt = "NumRange[]")]
  NumRangeArray,
  #[display(fmt = "TsRange")]
  TsRange,
  #[display(fmt = "TsRange[]")]
  TsRangeArray,
  #[display(fmt = "TstzRange")]
  TstzRange,
  #[display(fmt = "TstzRange[]")]
  TstzRangeArray,
  #[display(fmt = "DateRange")]
  DateRange,
  #[display(fmt = "DateRange[]")]
  DateRangeArray,
  #[display(fmt = "Int8Range")]
  Int8Range,
  #[display(fmt = "Int8Range[]")]
  Int8RangeArray,
  #[display(fmt = "Jsonpath")]
  Jsonpath,
  #[display(fmt = "Jsonpath[]")]
  JsonpathArray,
  #[display(fmt = "Regnamespace")]
  Regnamespace,
  #[display(fmt = "Regnamespace[]")]
  RegnamespaceArray,
  #[display(fmt = "Regrole")]
  Regrole,
  #[display(fmt = "Regrole[]")]
  RegroleArray,
  #[display(fmt = "Regcollation")]
  Regcollation,
  #[display(fmt = "Regcollation[]")]
  RegcollationArray,
  #[display(fmt = "PgMcvList")]
  PgMcvList,
  #[display(fmt = "PgSnapshot")]
  PgSnapshot,
  #[display(fmt = "PgSnapshot[]")]
  PgSnapshotArray,
  #[display(fmt = "Xid8")]
  Xid8,
  #[display(fmt = "Anycompatible")]
  Anycompatible,
  #[display(fmt = "Anycompatiblearray")]
  Anycompatiblearray,
  #[display(fmt = "Anycompatiblenonarray")]
  Anycompatiblenonarray,
  #[display(fmt = "AnycompatibleRange")]
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
