# Import Organization

krokfmt automatically organizes your imports to maintain consistency across your codebase.

## Import Categorization

Imports are categorized into three groups:

1. **External imports** - Packages from `node_modules`
2. **Absolute imports** - Paths starting with `@` or `~`
3. **Relative imports** - Paths starting with `./` or `../`

## Sorting Rules

Within each category, imports are sorted:

1. Alphabetically by package/path name
2. Default imports come before named imports
3. Type imports are treated the same as regular imports

## Example

### Before

```typescript
import { z } from 'zod';
import React from 'react';
import { Button } from './components/Button';
import { useAuth } from '@/hooks/auth';
import axios from 'axios';
import type { User } from '../types';
import './styles.css';
```

### After

```typescript
import axios from 'axios';
import React from 'react';
import { z } from 'zod';

import { useAuth } from '@/hooks/auth';

import './styles.css';
import { Button } from './components/Button';
import type { User } from '../types';
```

## Spacing

krokfmt automatically adds blank lines between import categories for better readability.

## Side-Effect Imports

Side-effect imports (like `import './styles.css'`) are kept in their respective categories and sorted with other imports.

## Type Imports

TypeScript type imports are treated the same as regular imports and sorted accordingly within their categories.