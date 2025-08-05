// FR1.1: Namespace imports should be parsed and preserved
import * as fs from 'fs';
import * as path from 'path';
import * as utils from './utils';
import * as helpers from '../helpers';

export function readFile(filename: string) {
    return fs.readFileSync(path.join(process.cwd(), filename));
}