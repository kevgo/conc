use cucumber::gherkin::Step;
use cucumber::{World as _, given, then, when};
use tokio::process::Command;

#[derive(Debug, Default, cucumber::World)]
struct World {
    workspace: Option<tempfile::TempDir>,
    output: Option<std::process::Output>,
    want_blocks: Vec<String>,
}

#[given("I'm in an empty folder")]
fn an_empty_folder(world: &mut World) {
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
    let mut command = Command::new(executable);
    command.args(args);
    if let Some(workspace) = &world.workspace {
        command.current_dir(workspace);
    }
    world.output = Some(command.output().await.unwrap());
}

#[then(expr = "the exit code is {int}")]
fn the_exit_code_is(world: &mut World, expected: i32) {
    let Some(output) = world.output.as_ref() else {
        panic!("No command ran yet");
    };
    assert_eq!(output.status.code().unwrap(), expected);
}

#[then("the output contains:")]
fn the_output_contains(world: &mut World, step: &Step) {
    let want = step.docstring().unwrap().trim();
    world.want_blocks.push(want.to_owned());
}

#[then("the output is:")]
fn the_output_is(world: &mut World, step: &Step) {
    let want = step.docstring().unwrap();
    let Some(output) = world.output.as_ref() else {
        panic!("No command ran yet");
    };
    let have = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    pretty::assert_eq!(have.trim(), want.trim());
    world.want_blocks.push(want.trim().to_string());
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    World::cucumber()
        .after(|_feature, _rule, _scenario, _ev, world| {
            Box::pin(async move {
                let Some(world) = world else {
                    panic!("No world");
                };
                let Some(output) = world.output.as_ref() else {
                    panic!("No command ran");
                };
                let mut have = format!(
                    "{}{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                for want in &world.want_blocks {
                    if !have.contains(want) {
                        panic!("Didn't find '{}' in output:\n{}", want, have);
                    }
                    have = have.replace(want, "");
                }
                have = have.trim().to_string();
                if !have.is_empty() {
                    panic!("Extra output found:\n{}", have);
                }
            })
        })
        .run_and_exit("features")
        .await;
}
