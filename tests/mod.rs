use progress_in_rust::{student, student::grade};

#[test]
fn test_student_struct() {
    let student_var: student::Student = student::Student {
        names: vec!["0", "1", "2"],
        email: "1",
        grades: vec![grade::Grades {
            assesment_name: "0",
            score: 0,
        }],
    };

    assert_eq!(
        format!("{}", student_var),
        "(names: [0, 1, 2], email: (1), grades: [(assesment_name: 0, score: 0)])"
    )
}
