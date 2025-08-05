// FR2.5: Dependencies should be grouped with their dependent exports for locality

// Dependencies scattered throughout the file
const helperA = () => "a";
const configB = { value: 42 };
const utilityC = (x: number) => x * 2;

// Export that uses helperA and configB
export function serviceA() {
    return helperA() + configB.value;
}

// Another dependency
const dataD = [1, 2, 3];

// Export that uses utilityC and dataD
export function serviceB() {
    return dataD.map(utilityC);
}

// Shared dependency
const sharedLogger = {
    log: (msg: string) => console.log(msg)
};

// Multiple exports using shared dependency
export function featureX() {
    sharedLogger.log("featureX");
    return helperA();
}

export function featureY() {
    sharedLogger.log("featureY");
    return configB.value;
}

// Export statement depending on non-exported members
export { helperA, configB };

// Independent export with no dependencies
export function independentService() {
    return "no dependencies";
}

// Non-exported helper that no export depends on
function unusedHelper() {
    return "not used by exports";
}