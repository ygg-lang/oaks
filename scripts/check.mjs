import { spawn } from "node:child_process"
import { platform } from "node:os"

const isWindows = platform() === "win32"
const shell = isWindows ? true : false

const colors = {
    reset: "\x1b[0m",
    red: "\x1b[31m",
    green: "\x1b[32m",
    yellow: "\x1b[33m",
    blue: "\x1b[34m",
    cyan: "\x1b[36m",
}

function log(color, message) {
    console.log(`${colors[color]}${message}${colors.reset}`)
}

function runCommand(command, args = [], options = {}) {
    return new Promise((resolve, reject) => {
        const child = spawn(command, args, {
            shell,
            stdio: "inherit",
            ...options,
        })

        child.on("close", (code) => {
            if (code === 0) {
                resolve(code)
            } else {
                reject(new Error(`Command failed with exit code ${code}`))
            }
        })

        child.on("error", (err) => {
            reject(err)
        })
    })
}

async function runStep(name, fn) {
    log("cyan", `\n${"=".repeat(60)}`)
    log("cyan", `  ${name}`)
    log("cyan", `${"=".repeat(60)}\n`)

    try {
        await fn()
        log("green", `\n[PASS] ${name}\n`)
        return true
    } catch (error) {
        log("red", `\n[FAIL] ${name}`)
        if (error.message) {
            log("red", `  Error: ${error.message}`)
        }
        return false
    }
}

async function rustFormatCheck() {
    await runCommand("cargo", ["fmt", "--all", "--", "--check"])
}

async function rustClippy() {
    await runCommand("cargo", ["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"])
}

async function rustCheck() {
    await runCommand("cargo", ["check", "--all-targets", "--all-features"])
}

async function rustTestNoDefault() {
    await runCommand("cargo", ["test", "--workspace", "--no-default-features"])
}

async function rustTestDefault() {
    await runCommand("cargo", ["test", "--workspace"])
}

async function rustTestAllFeatures() {
    await runCommand("cargo", ["test", "--workspace", "--all-features"])
}

async function main() {
    log("blue", "\n╔══════════════════════════════════════════════════════════╗")
    log("blue", "║               Local CI Check Runner                      ║")
    log("blue", "╚══════════════════════════════════════════════════════════╝\n")

    const results = []

    const rustChecks = [
        { name: "Rust Format Check", fn: rustFormatCheck },
        { name: "Rust Clippy", fn: rustClippy },
        { name: "Rust Check Compilation", fn: rustCheck },
    ]

    const rustTests = [
        { name: "Rust Test (no-default features)", fn: rustTestNoDefault },
        { name: "Rust Test (default features)", fn: rustTestDefault },
        { name: "Rust Test (all features)", fn: rustTestAllFeatures },
    ]

    log("yellow", "▶ Running Rust Checks...")
    for (const check of rustChecks) {
        const passed = await runStep(check.name, check.fn)
        results.push({ name: check.name, passed, category: "Rust Check" })
    }

    log("yellow", "\n▶ Running Rust Tests...")
    for (const test of rustTests) {
        const passed = await runStep(test.name, test.fn)
        results.push({ name: test.name, passed, category: "Rust Test" })
    }

    log("blue", "\n╔══════════════════════════════════════════════════════════╗")
    log("blue", "║                      Summary                              ║")
    log("blue", "╚══════════════════════════════════════════════════════════╝\n")

    const passed = results.filter((r) => r.passed)
    const failed = results.filter((r) => !r.passed)

    for (const r of results) {
        const status = r.passed ? `${colors.green}✓ PASS${colors.reset}` : `${colors.red}✗ FAIL${colors.reset}`
        console.log(`  ${status}  ${r.name}`)
    }

    console.log("")
    log("cyan", `Total: ${results.length} | ${colors.green}Passed: ${passed.length}${colors.reset} | ${colors.red}Failed: ${failed.length}${colors.reset}`)

    if (failed.length > 0) {
        log("red", "\n❌ CI check failed!\n")
        process.exit(1)
    } else {
        log("green", "\n✅ All CI checks passed!\n")
        process.exit(0)
    }
}

main().catch((err) => {
    log("red", `\nFatal error: ${err.message}`)
    process.exit(1)
})
