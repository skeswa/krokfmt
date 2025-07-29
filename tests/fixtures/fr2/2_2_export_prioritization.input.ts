// FR2.2: Exported members should be prioritized over non-exported

// Mix of exported and non-exported members
const privateConfig = { debug: false };

export const publicConfig = { api: 'https://api.example.com' };

function privateHelper() {
    return 'private';
}

export function publicHelper() {
    return 'public';
}

interface PrivateOptions {
    internal: boolean;
}

export interface PublicOptions extends PrivateOptions {
    external: boolean;
}

class InternalService {
    name = 'internal';
}

export class PublicService {
    service = new InternalService();
}

// Multiple exports should all be prioritized
export const constant1 = 1;
const privateConstant = 2;
export const constant2 = 3;
const anotherPrivate = 4;
export const constant3 = 5;