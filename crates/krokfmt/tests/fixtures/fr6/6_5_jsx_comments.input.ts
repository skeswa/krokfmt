// FR6.5: Comments in JSX should be preserved

import React from 'react';

export function UserProfile({ user }) {
    return (
        <div
            className="profile" // Main container
            id={`user-${user.id}`} // Dynamic ID
        >
            {/* User header section */}
            <header>
                <h1>{user.name}</h1> {/* Display name */}
                <p>{user.bio}</p>
            </header>
            
            {/* User actions */}
            <div className="actions">
                <button
                    onClick={handleEdit} // Edit handler
                    disabled={!canEdit} // Disable if no permission
                >
                    Edit Profile
                </button>
            </div>
        </div>
    );
}