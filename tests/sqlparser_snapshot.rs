// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![warn(clippy::all)]

use sqlparser::ast::Statement;
use sqlparser::dialect::{Dialect, GenericDialect};
use sqlparser::parser::Parser;
use std::path::Path;

fn test_dialect(path: &Path, dialect: &dyn Dialect) -> datatest_stable::Result<()> {
    let sql = std::fs::read_to_string(path)?;

    let stmts = Parser::parse_sql(dialect, &sql)?;
    let name = path
        .strip_prefix(Path::new("tests/queries"))?
        .to_str()
        .unwrap();
    insta::assert_yaml_snapshot!(name, stmts);

    // Rendering the AST to a string and re-parsing it should yield the same AST.
    for stmt in stmts {
        let [round_trip_stmt]: [Statement; 1] = Parser::parse_sql(dialect, &stmt.to_string())?
            .try_into()
            .map_err(|_| "expected a single statement")?;
        pretty_assertions::assert_eq!(stmt, round_trip_stmt);
    }

    Ok(())
}

fn test_generic(path: &Path) -> datatest_stable::Result<()> {
    test_dialect(path, &GenericDialect)
}

datatest_stable::harness!(test_generic, "tests/queries/generic", r"^.*.sql",);
