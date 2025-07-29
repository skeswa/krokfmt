// FR1.1: Import aliases should be parsed and preserved
import { foo as bar } from './module';
import { Component as MyComponent } from '@ui/components';
import { default as DefaultExport } from './default';
import React, { Component as ReactComponent } from 'react';

export const App = () => {
    return <MyComponent />;
};