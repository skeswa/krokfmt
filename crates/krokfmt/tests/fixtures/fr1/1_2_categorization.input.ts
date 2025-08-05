// FR1.2: Imports should be categorized into External, Absolute, and Relative groups
import { helper } from './utils/helper';
import React from 'react';
import { Button } from '@components/Button';
import axios from 'axios';
import { api } from '@services/api';
import { User } from '../types/user';
import { theme } from '~/theme/default';
import lodash from 'lodash';
import { local } from './local';

export const App = () => {
    return <Button>Click me</Button>;
};