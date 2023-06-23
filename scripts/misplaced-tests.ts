/**
 * This script checks for invalid test placements in the source code.
 * It identifies cases where #[cfg(test)] mod tests or #[test] functions
 * are defined within 'src' directories instead of the recommended 'tests' directory.
 */

import * as fs from 'node:fs';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';

function walk(dir: string, callback: (filePath: string) => void) {
    const files = fs.readdirSync(dir);
    for (const file of files) {
        const fullPath = path.join(dir, file);
        const stat = fs.statSync(fullPath);
        if (stat.isDirectory()) {
            if (['node_modules', 'target', '.git'].includes(file)) continue;
            walk(fullPath, callback);
        } else {
            callback(fullPath);
        }
    }
}

function checkInvalidTests(rootDir: string) {
    console.log(`Checking for invalid test placements in ${rootDir}...`);
    let count = 0;

    walk(rootDir, (filePath) => {
        if (!filePath.endsWith('.rs')) return;

        // 仅检查 src 目录下的文件
        const pathParts = filePath.split(path.sep);
        if (!pathParts.includes('src')) return;

        const content = fs.readFileSync(filePath, 'utf8');
        const lines = content.split('\n');
        
        let inQuote = false;

        lines.forEach((line, index) => {
            // 简单的 quote! 宏识别
            if (line.includes('quote! {')) inQuote = true;
            if (inQuote && line.includes('}')) {
                const trimmed = line.trim();
                if (trimmed === '};' || trimmed === '}') inQuote = false;
            }

            if (!inQuote) {
                const trimmed = line.trim();
                if (trimmed.startsWith('#[cfg(test)]') || trimmed.startsWith('#[test]')) {
                    const nextLine = (lines[index + 1] || '').trim();
                    if (trimmed.includes('mod tests') || nextLine.startsWith('mod tests') || 
                        trimmed.includes('fn ') || nextLine.startsWith('fn ')) {
                        console.log(`Invalid test placement: ${filePath}:${index + 1}`);
                        count++;
                    }
                }
            }
        });
    });

    console.log(`\nFound ${count} invalid test placements.`);
}

const root = path.resolve(fileURLToPath(import.meta.url), '..', '..');
checkInvalidTests(root);
