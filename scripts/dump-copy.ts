import { cp, rm, writeFile } from 'node:fs/promises';

const dump_dir = '/disk2/dump-files/dumpar';
const dev_dir = '/home/ali/dev';

const dirs = {
	'tests/gramex': 'gramex/tests',
	'tests/mep': 'mep/tests',
	'tests/neocomp': 'neocomp/test',
	'tests/neoview': 'neoview/tests',
	scripts: 'scripts',
	'tests/structom': 'structom/test',
	'tests/ts-madness': 'ts-madness/test',
	'uni-hw': 'uni-hw',
};

for (let [dir, src] of Object.entries(dirs)) {
	await rm(`${dump_dir}/${dir}`, { recursive: true, force: true });
	await cp(`${dev_dir}/${src}`, `${dump_dir}/${dir}`, { recursive: true });
	writeFile(`${dump_dir}/${dir}/.origin`, src);
}
