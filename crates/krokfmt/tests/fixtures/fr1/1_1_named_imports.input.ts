// FR1.1: Named imports should be parsed and preserved
import { useState, useEffect } from 'react';
import { debounce, throttle } from 'lodash';
import { AxiosError, AxiosResponse } from 'axios';

function useApi() {
    const [data, setData] = useState(null);
    return data;
}