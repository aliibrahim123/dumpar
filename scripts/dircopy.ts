import { readdir, readFile } from 'node:fs/promises';
import { spawn } from 'node:child_process';

let result = '';
let files: string[] = [];

async function walk(path: string, include: string, exclude: string) {
	for (let entry of await readdir(path, { withFileTypes: true }).catch(() => [])) {
		let file_path = path + '/' + entry.name;
		if (entry.isDirectory()) await walk(file_path, include, exclude);
		else if (entry.isFile()) {
			if (!file_path.match(include) || file_path.match(exclude)) continue;
			files.push(file_path);
			result += `==== file: ${file_path} ====\n`;
			try {
				result += await readFile(file_path, 'utf-8');
			} catch (e) {}
			result += '\n\n';
		}
	}
}

await walk('.', process.argv[2] || '.*', process.argv[3] || '\uffff\uffff\uffff');
console.log(result);
console.log('processed files:');
console.log(files.join('\n'));

let proc = spawn('wl-copy', [], { stdio: ['pipe', 'inherit', 'inherit'] });
proc.stdin.write(result);
proc.stdin.end();
