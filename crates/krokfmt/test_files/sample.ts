// Sample TypeScript file to test krokfmt
import { helper } from './utils/helper';
import React, { useState, useEffect } from 'react';
import { Button, TextField } from '@mui/material';
import axios from 'axios';
import { API_BASE_URL } from '@config/constants';
import type { User, Profile } from '../types/user';
import lodash from 'lodash';

interface Props {
    title: string;
    onClose: () => void;
    userId: number;
    isActive: boolean;
}

export const UserComponent: React.FC<Props> = ({
    userId,
    title,
    isActive,
    onClose,
}) => {
    const [user, setUser] = useState<User | null>(null);
    const [loading, setLoading] = useState(false);

    const config = {
        timeout: 5000,
        headers: {
            'Authorization': 'Bearer token',
            'Content-Type': 'application/json',
        },
        baseURL: API_BASE_URL,
    };

    useEffect(() => {
        fetchUser();
    }, [userId]);

    const fetchUser = async () => {
        try {
            setLoading(true);
            const response = await axios.get(`/users/${userId}`, config);
            setUser(response.data);
        } catch (error) {
            console.error('Error fetching user:', error);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div>
            <h1>{title}</h1>
            {loading ? (
                <p>Loading...</p>
            ) : (
                <div>
                    <p>User: {user?.name}</p>
                    <Button onClick={onClose} disabled={!isActive}>
                        Close
                    </Button>
                </div>
            )}
        </div>
    );
};

const utils = {
    formatDate: (date: Date) => date.toISOString(),
    parseUser: (data: any) => data as User,
    validateEmail: (email: string) => /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email),
};

export default UserComponent;