// FR1.4: All imports should be positioned at the top of the file

// This variable is declared before imports - should be moved after
const earlyVar = 'I should be after imports';

// Another early declaration
function earlyFunction() {
    return 'Also should be after imports';
}

import React from 'react';
import { useState } from 'react';

// More code mixed with imports
const middleVar = 'mixed';

import axios from 'axios';
import { Button } from '@ui/Button';

// Final code
export const App = () => {
    return <Button onClick={() => console.log(earlyVar)}>Click</Button>;
};