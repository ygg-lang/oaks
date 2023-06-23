/**
 * This script checks for Rust source files (.rs) that exceed a specified line count (default 1000).
 * It helps identify files that may need refactoring due to excessive complexity or size.
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

function checkFileSize(rootDir: string, maxLines: number = 1000) {
    console.log(`Checking for Rust files larger than ${maxLines} lines in ${rootDir}...`);
    let count = 0;

    walk(rootDir, (filePath) => {
        if (!filePath.endsWith('.rs')) return;

        const content = fs.readFileSync(filePath, 'utf8');
        const lines = content.split('\n');
        
        if (lines.length > maxLines) {
            console.log(`Large file found: ${filePath} (${lines.length} lines)`);
            count++;
        }
    });

    console.log(`\nFound ${count} large files.`);
}

const root = path.resolve(fileURLToPath(import.meta.url), '..', '..');
checkFileSize(root);
