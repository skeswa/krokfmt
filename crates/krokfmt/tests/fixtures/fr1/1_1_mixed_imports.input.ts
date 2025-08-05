// FR1.1: Mixed imports should be parsed and preserved
import React, { useState, useEffect } from 'react';
import axios, { AxiosError } from 'axios';
import utils, { helper, type HelperType } from '../utils/helpers';

export function useData() {
    const [data, setData] = useState(null);
    useEffect(() => {
        axios.get('/api/data').then(res => setData(res.data));
    }, []);
    return data;
}