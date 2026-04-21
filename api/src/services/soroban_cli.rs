use std::process::Command;

pub fn invoke(
  contract_id: &str,
  source: &str,
  network: &str,
  function: &str,
  args: Vec<(&str, String)>,
) -> Result<String, String> {
  let mut cmd = Command::new("stellar");

  cmd.args([
    "contract", "invoke",
    "--id", contract_id,
    "--source", source,
    "--network", network,
    "--",
    function,
  ]);

  for (key, value) in args {
    cmd.arg(key).arg(value);
  }

  let output = cmd.output().map_err(|e| e.to_string())?;

  if output.status.success() {
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
  } else {
    Err(String::from_utf8_lossy(&output.stderr).to_string())
  }
}
