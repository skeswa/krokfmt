// FR2.3: Namespace and module dependencies

// Internal namespace
namespace Internal {
    export interface Config {
        timeout: number;
    }
    
    export const defaultConfig: Config = {
        timeout: 5000
    };
}

// Exported namespace depending on internal
export namespace Api {
    export interface Options extends Internal.Config {
        retries: number;
    }
    
    export const defaultOptions: Options = {
        ...Internal.defaultConfig,
        retries: 3
    };
}

// Type dependencies
type Handler = (data: any) => void;
type AsyncHandler = (data: any) => Promise<void>;

export type Middleware = {
    before: Handler;
    after: AsyncHandler;
};

// Const assertions
const MODES = ['dev', 'prod'] as const;
type Mode = typeof MODES[number];

export const currentMode: Mode = MODES[0];