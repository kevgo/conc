use cucumber::gherkin::Step;
use cucumber::{World as _, given, then, when};
use tokio::process::Command;

#[derive(Debug, Default, cucumber::World)]
struct World {
    workspace: Option<tempfile::TempDir>,
    output: Option<std::process::Output>,
}

#[given("an empty folder")]
async fn an_empty_folder(world: &mut World) {
    world.workspace = Some(tempfile::tempdir().unwrap());
}

#[when(expr = "I run {string}")]
async fn i_run(world: &mut World, command: String) {
    let mut args = shellwords::split(&command).unwrap().into_iter();
    let mut executable = args.next().unwrap();
    if executable == "conc" {
        let cwd = std::env::current_dir().unwrap();
        let conc_path = cwd.join("target/debug/conc");
        executable = conc_path.to_string_lossy().to_string();
    }
    world.output = Some(Command::new(executable).args(args).output().await.unwrap());
}

#[then("the output contains:")]
async fn the_output_contains(world: &mut World, step: &Step) {
    let Some(output) = world.output.as_ref() else {
        panic!("No command ran yet");
    };
    let have = String::from_utf8_lossy(&output.stdout) + String::from_utf8_lossy(&output.stderr);
    let want = step.docstring().unwrap().trim();
    assert!(
        have.contains(want),
        "Didn't find '{}' in output:\n{}",
        want,
        have
    );
}
#[then("the output contains these lines in any order:")]
async fn the_output_contains_these_lines_in_any_order(world: &mut World, step: &Step) {
    let Some(output) = world.output.as_ref() else {
        panic!("No command ran yet");
    };
    let output_text =
        String::from_utf8_lossy(&output.stdout) + String::from_utf8_lossy(&output.stderr);
    let have = output_text.lines().collect::<Vec<_>>();
    let want_text = step.docstring().unwrap().trim();
    let want = want_text.lines().collect::<Vec<_>>();
    assert_eq!(
        have.len(),
        want.len(),
        "Expected {} lines in output, got {}",
        want.len(),
        have.len()
    );
    for line in want {
        assert!(
            have.contains(&line),
            "Didn't find '{}' in output:\n{}",
            line,
            output_text
        );
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    World::run("features").await;
}
