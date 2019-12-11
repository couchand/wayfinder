use prettydiff::text::{diff_lines, LineChangeset};
use prettydiff::basic::DiffOp;

pub struct TestResult<'a> {
    expected: &'a str,
    actual: &'a str,
}

impl<'a> TestResult<'a> {
    pub fn new(expected: &'a str, actual: &'a str) -> TestResult<'a> {
        TestResult { expected, actual }
    }

    pub fn assert(&self) {
        if self.expected != self.actual {
            let diff = TestDiff {
                changes: diff_lines(self.expected, self.actual),
                old_name: "expected",
                new_name: "actual",
            };
            eprintln!("{}", diff);
            assert!(false, "Output does not match expected; see diff above.");
        }
    }
}

struct DiffChunk<'source, 'changes> {
    old_name: &'changes str,
    old_line: usize,
    old_count: usize,
    new_name: &'changes str,
    new_line: usize,
    new_count: usize,
    diff: Vec<DiffOp<'source, &'changes str>>,
}

impl<'source, 'changes> std::fmt::Display for DiffChunk<'source, 'changes> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "--- {}", self.old_name)?;
        writeln!(f, "+++ {}", self.new_name)?;
        writeln!(f, "@@ -{},{} +{},{} @@", self.old_line, self.old_count, self.new_line, self.new_count)?;

        for op in self.diff.iter() {
            match op {
                DiffOp::Equal(_) => {},
                DiffOp::Insert(a) => writeln!(f, "+ {}", a.join("\n+ "))?,
                DiffOp::Remove(a) => writeln!(f, "- {}", a.join("\n- "))?,
                DiffOp::Replace(a, b) => {
                    writeln!(f, "- {}", a.join("\n- "))?;
                    writeln!(f, "+ {}", b.join("\n+ "))?;
                }
            }
        }

        Ok(())
    }
}

struct TestDiff<'a> {
    changes: LineChangeset<'a>,
    old_name: &'a str,
    new_name: &'a str,
}

impl<'a> TestDiff<'a> {
    fn to_chunks<'b>(&'b self) -> Vec<DiffChunk<'a, 'b>> {
        let mut chunks = vec![];
        let mut old_line = 1;
        let mut new_line = 1;

        for op in self.changes.diff() {
            match op {
                DiffOp::Equal(a) => {
                    let count = a.len();
                    old_line += count;
                    new_line += count;
                    // TODO: context
                },
                DiffOp::Insert(a) => {
                    let new_count = a.len();
                    chunks.push(DiffChunk {
                        old_name: self.old_name,
                        old_line,
                        old_count: 0,
                        new_name: self.new_name,
                        new_line,
                        new_count,
                        diff: vec![DiffOp::Insert(a)], // TODO: chunk
                    });
                    new_line += new_count;
                },
                DiffOp::Remove(a) => {
                    let old_count = a.len();
                    chunks.push(DiffChunk {
                        old_name: self.old_name,
                        old_line,
                        old_count,
                        new_name: self.new_name,
                        new_line,
                        new_count: 0,
                        diff: vec![DiffOp::Remove(a)], // TODO: chunk
                    });
                    old_line += old_count;
                },
                DiffOp::Replace(a, b) => {
                    let old_count = a.len();
                    let new_count = b.len();
                    chunks.push(DiffChunk {
                        old_name: self.old_name,
                        old_line,
                        old_count,
                        new_name: self.new_name,
                        new_line,
                        new_count,
                        diff: vec![DiffOp::Replace(a, b)], // TODO: chunk
                    });
                    old_line += old_count;
                    new_line += new_count;
                },
            }
        }

        chunks
    }
}

impl<'a> std::fmt::Display for TestDiff<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for chunk in self.to_chunks() {
            writeln!(f, "{}", chunk)?;
        }

        Ok(())
    }
}
