// FR2.4: Test case-insensitive alphabetization within visibility groups

// Exported members with mixed case
export function validateUser() { return true; }
export class ApiHandler { handle() {} }
export interface UserProfile { name: string; }
export function authenticateUser() { return true; }
export const VERSION = '1.0.0';
export type UserId = string;
export enum LogLevel { INFO, ERROR }
export class Database { connect() {} }
export const apiKey = 'secret';

// Non-exported members with mixed case
function processData() { return []; }
class WebSocket { send() {} }
interface ResponseData { status: number; }
function analyzeResults() { return {}; }
const MAX_RETRIES = 3;
type SessionId = string;
enum Priority { LOW, HIGH }
class Analytics { track() {} }
const baseUrl = 'http://localhost';