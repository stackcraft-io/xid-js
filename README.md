# xid-js

## Install

```
pnpm|yarn|npm add @stackcraft.io/xid-js
```

## Usage

```ts
import * as xid from '@stackcraft.io/xid-js';

const id = xid.one();

const [userId, teamId] = xid.many(2);

```