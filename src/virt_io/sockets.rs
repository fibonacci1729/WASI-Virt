use anyhow::{bail, Context, Result};
use walrus::Module;

use super::StubRequirement;

/// Imports exposed by WASI for sockets functionality which are allowed to be missing
const WASI_SOCKETS_IMPORTS: [(&str, &str, &StubRequirement); 49] = [
    (
        "wasi:sockets/ip-name-lookup@0.2.0-rc-2023-10-18",
        "resolve-addresses",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/ip-name-lookup@0.2.0-rc-2023-10-18",
        "[method]resolve-address-stream.resolve-next-address",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/ip-name-lookup@0.2.0-rc-2023-10-18",
        "[resource-drop]resolve-address-stream",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/ip-name-lookup@0.2.0-rc-2023-10-18",
        "[method]resolve-address-stream.subscribe",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.start-bind",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.finish-bind",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.start-connect",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.finish-connect",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.start-listen",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.finish-listen",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.accept",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.local-address",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.remote-address",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.address-family",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.ipv6-only",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.set-ipv6-only",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.set-listen-backlog-size",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.keep-alive",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.set-keep-alive",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.no-delay",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.set-no-delay",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.unicast-hop-limit",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.set-unicast-hop-limit",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.receive-buffer-size",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.set-receive-buffer-size",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.send-buffer-size",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.set-send-buffer-size",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.subscribe",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[method]tcp-socket.shutdown",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/tcp@0.2.0-rc-2023-10-18",
        "[resource-drop]tcp-socket",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.start-bind",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.finish-bind",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.start-connect",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.finish-connect",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.receive",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.send",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.local-address",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.remote-address",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.address-family",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.ipv6-only",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.set-ipv6-only",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.unicast-hop-limit",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.set-unicast-hop-limit",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.receive-buffer-size",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.set-receive-buffer-size",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.send-buffer-size",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.set-send-buffer-size",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[method]udp-socket.subscribe",
        &StubRequirement::Required,
    ),
    (
        "wasi:sockets/udp@0.2.0-rc-2023-10-18",
        "[resource-drop]udp-socket",
        &StubRequirement::Required,
    ),
];

/// Replace imported WASI functions that implement socket access with no-ops
pub(crate) fn stub_sockets_virt(module: &mut Module) -> Result<()> {
    for (module_name, func_name, stub_requirement) in WASI_SOCKETS_IMPORTS {
        match stub_requirement {
            StubRequirement::Required => {
                let fid = module
                    .imports
                    .get_func(module_name, func_name)
                    .with_context(|| {
                        format!(
                    "failed to find required sockets import [{func_name}] in module [{module_name}]"
                )
                    })?;
                module
                    .replace_imported_func(fid, |(body, _)| {
                        body.unreachable();
                    })
                    .with_context(|| {
                        "failed to stub sockets functionality [{}] in module [{export_name}]"
                    })?;
            }
            _ => bail!("unexpected stub requirement in imports for WASI sockets"),
        }
    }

    Ok(())
}

/// Exported functions related to sockets
const WASI_SOCKETS_EXPORTS: [&str; 49] = [
    "wasi:sockets/ip-name-lookup@0.2.0-rc-2023-10-18#resolve-addresses",
    "wasi:sockets/ip-name-lookup@0.2.0-rc-2023-10-18#[method]resolve-address-stream.resolve-next-address",
    "wasi:sockets/ip-name-lookup@0.2.0-rc-2023-10-18#[dtor]resolve-address-stream",
    "wasi:sockets/ip-name-lookup@0.2.0-rc-2023-10-18#[method]resolve-address-stream.subscribe",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.start-bind",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.finish-bind",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.start-connect",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.finish-connect",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.start-listen",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.finish-listen",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.accept",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.local-address",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.remote-address",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.address-family",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.ipv6-only",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.set-ipv6-only",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.set-listen-backlog-size",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.keep-alive",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.set-keep-alive",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.no-delay",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.set-no-delay",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.unicast-hop-limit",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.set-unicast-hop-limit",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.receive-buffer-size",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.set-receive-buffer-size",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.send-buffer-size",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.set-send-buffer-size",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.subscribe",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[method]tcp-socket.shutdown",
    "wasi:sockets/tcp@0.2.0-rc-2023-10-18#[dtor]tcp-socket",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.start-bind",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.finish-bind",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.start-connect",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.finish-connect",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.receive",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.send",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.local-address",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.remote-address",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.address-family",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.ipv6-only",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.set-ipv6-only",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.unicast-hop-limit",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.set-unicast-hop-limit",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.receive-buffer-size",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.set-receive-buffer-size",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.send-buffer-size",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.set-send-buffer-size",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[method]udp-socket.subscribe",
    "wasi:sockets/udp@0.2.0-rc-2023-10-18#[dtor]udp-socket",
];

/// Strip exported WASI functions that implement sockets access
pub(crate) fn strip_sockets_virt(module: &mut Module) -> Result<()> {
    stub_sockets_virt(module)?;
    for export_name in WASI_SOCKETS_EXPORTS {
        module.exports.remove(export_name).with_context(|| {
            format!("failed to strip WASI sockets export function [{export_name}]")
        })?;
    }
    Ok(())
}
