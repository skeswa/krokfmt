// FR1.6: Import syntax should be preserved exactly
import data from './data.json' assert { type: 'json' };
import styles from './styles.css' assert { type: 'css' };
import MyReact from 'react';
import type { User } from './types';
import { type Config, type Settings } from './config';
import './side-effect';
import 'global-polyfill';

export { default as MyExport } from './my-export';
export * from './re-exports';
export { foo, bar } from './module';