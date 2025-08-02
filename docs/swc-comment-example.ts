// This file demonstrates how SWC stores comments with byte positions marked

// Byte positions (for reference):
// 000000000111111111122222222223333333333444444444455555555556666666666
// 012345678901234567890123456789012345678901234567890123456789012345678

// Leading comment for import
import { foo } from './foo'; // Trailing on import

/* Block comment
   spanning multiple lines */
// Another leading comment
export const bar = 42; // Trailing on export

// Function with internal comments
function example() {
    // Comment inside function
    const x = 1;
    
    /* Multi-line inside function
       with more text */
    return x;
}

// Class with various comment positions
class Demo {
    // Comment on property
    name: string;
    
    // Comment on method
    method() {
        // Internal method comment
    }
}

/*
 * Detailed byte position analysis:
 * 
 * Line 7: "// Leading comment for import"
 *   - Comment spans bytes 116-145
 *   - But stored at BytePos(146) where "import" starts
 * 
 * Line 8: "import { foo } from './foo';"
 *   - Code spans bytes 146-174
 *   - Leading comment at BytePos(146)
 *   - Trailing comment at BytePos(174)
 * 
 * Line 8: "// Trailing on import"
 *   - Comment spans bytes 175-196
 *   - Stored as trailing at BytePos(174)
 * 
 * Lines 10-11: Block comment
 *   - Comment spans bytes 198-238
 *   - Stored at BytePos(268) where "export" starts
 * 
 * Key insight: Comments are ALWAYS stored at code positions,
 * never at their actual location in the source!
 */