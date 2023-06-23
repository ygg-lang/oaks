import { spawn } from 'node:child_process';
import * as fs from 'node:fs';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(__dirname, '..');
const logFile = path.resolve(rootDir, 'all-errors.log');

interface CargoMessage {
    reason: string;
    message?: {
        level: string;
        message: string;
        code?: { code: string };
        spans: Array<{
            file_name: string;
            line_start: number;
            column_start: number;
        }>;
    };
}

async function runCargoCheck(args: string[]): Promise<string[]> {
    return new Promise((resolve) => {
        const errors: string[] = [];
        const child = spawn('cargo', ['check', '--message-format=json', ...args], {
            cwd: rootDir,
            shell: true,
        });

        let buffer = '';
        child.stdout.on('data', (data) => {
            buffer += data.toString();
            const lines = buffer.split('\n');
            buffer = lines.pop() || '';

            for (const line of lines) {
                if (!line.trim()) continue;
                try {
                    const msg: CargoMessage = JSON.parse(line);
                    if (msg.reason === 'compiler-message' && msg.message?.level === 'error') {
                        const message = msg.message;
                        const code = message.code ? ` [${message.code.code}]` : '';
                        const spans = message.spans;
                        if (spans && spans.length > 0) {
                            const span = spans[0];
                            errors.push(`${span.file_name}:${span.line_start}:${span.column_start}: error:${code} ${message.message}`);
                        } else {
                            errors.push(`global: error:${code} ${message.message}`);
                        }
                    }
                } catch (e) {
                    // Ignore non-json lines
                }
            }
        });

        child.on('close', () => {
            resolve(errors);
        });
    });
}

async function main() {
    console.log('\n🚀 Starting Cargo Multi-Config Check\n' + '━'.repeat(50));
    
    const configs = [
        { name: 'no-features', args: ['--no-default-features'], emoji: '👻' },
        { name: 'default', args: [], emoji: '📦' },
        { name: 'all-features', args: ['--all-features'], emoji: '🌟' },
    ];

    let allErrors: string[] = [];
    const maxNameLen = Math.max(...configs.map(c => c.name.length));

    for (const config of configs) {
        const paddedName = config.name.padEnd(maxNameLen);
        process.stdout.write(`${config.emoji} Checking ${paddedName} ... `);
        
        const errors = await runCargoCheck(config.args);
        
        if (errors.length > 0) {
            console.log(`❌ found ${errors.length.toString().padStart(2)} errors`);
            allErrors.push(`=== 🛠️ Configuration: ${config.name} ===`);
            allErrors.push(...errors);
            allErrors.push('');
        } else {
            console.log('✅ clean');
        }
    }

    console.log('━'.repeat(50));

    if (allErrors.length > 0) {
        fs.writeFileSync(logFile, allErrors.join('\n'));
        console.log(`📝 Log: ${logFile}`);
        console.log('👀 Please fix the errors listed above!\n');
    } else {
        if (fs.existsSync(logFile)) fs.unlinkSync(logFile);
        console.log('🎉 All configurations passed! You are a genius! 🌈\n');
    }
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});
