// FR1.3: Test case-insensitive alphabetical sorting of imports

// External imports with mixed case
import zod from 'zod';
import React from 'react';
import axios from 'axios';
import VueRouter from 'vue-router';
import express from 'express';
import { Component } from 'react';
import lodash from 'lodash';
import NextAuth from 'next-auth';

// Absolute imports with mixed case
import { Button } from '@UI/Button';
import { helper } from '@utils/helper';
import { ApiClient } from '@services/ApiClient';
import { validate } from '@utils/validate';
import { Modal } from '@components/Modal';
import { useAuth } from '@hooks/useAuth';
import { CONSTANTS } from '@config/CONSTANTS';

// Relative imports with mixed case
import { Widget } from './components/Widget';
import { api } from './api';
import { Store } from '../store/Store';
import { database } from '../db/database';
import { Analytics } from './services/Analytics';
import { logger } from './utils/logger';
import { Config } from '../config/Config';