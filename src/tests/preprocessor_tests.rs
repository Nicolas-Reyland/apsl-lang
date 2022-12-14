// Tests for preprocessor

#[cfg(test)]

use crate::preprocessor::remove_comments;

#[test]
fn test_remove_comments() {
    let src = "/* start comment */
this is code // and this is not */
I/**/just/**/ want to // test things
// break things
do stuff
//
";
    assert_eq!(
        remove_comments(src),
        "\n".to_string() +
        "this is code \n" +
        "Ijust want to \n" +
        "\n" +
        "do stuff\n" +
        "\n"
    );
}
