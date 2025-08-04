use criterion::{black_box, criterion_group, criterion_main, Criterion};
use krokfmt::{codegen::CodeGenerator, organizer::KrokOrganizer, parser::TypeScriptParser};

fn organize_code(input: &str) -> String {
    let parser = TypeScriptParser::new();
    let source_map = parser.source_map.clone();
    let comments = parser.comments.clone();
    let filename = if input.contains("<") && input.contains(">") {
        "bench.tsx"
    } else {
        "bench.ts"
    };
    let module = parser.parse(input, filename).unwrap();
    let organizer = KrokOrganizer::new();
    let organized_module = organizer.organize(module).unwrap();
    let generator = CodeGenerator::with_comments(source_map, comments);
    generator.generate(&organized_module).unwrap()
}

fn bench_small_file(c: &mut Criterion) {
    let input = r#"
import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import * as moment from 'moment';
import React from 'react';
import { useState, useEffect } from 'react';

export class AppComponent {
    title = 'app';
    
    constructor(private http: HttpClient) {}
    
    getData() {
        return this.http.get('/api/data');
    }
}
"#;

    c.bench_function("format_small_file", |b| {
        b.iter(|| organize_code(black_box(input)))
    });
}

fn bench_medium_file(c: &mut Criterion) {
    let input = r#"
import { Component, OnInit, OnDestroy } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, Subject } from 'rxjs';
import { takeUntil, map, filter } from 'rxjs/operators';
import * as moment from 'moment';
import React from 'react';
import { useState, useEffect, useCallback } from 'react';
import { useRouter } from 'next/router';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Dialog } from '@/components/ui/Dialog';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { api } from './services/api';
import { auth } from './services/auth';
import { config } from './config';
import type { User, Post, Comment } from './types';

export class AppComponent implements OnInit, OnDestroy {
    title = 'app';
    users: User[] = [];
    posts: Post[] = [];
    comments: Comment[] = [];
    loading = false;
    error: string | null = null;
    private destroy$ = new Subject<void>();
    
    constructor(
        private http: HttpClient,
        private authService: auth,
        private apiService: api
    ) {}
    
    ngOnInit() {
        this.loadData();
        this.setupSubscriptions();
    }
    
    ngOnDestroy() {
        this.destroy$.next();
        this.destroy$.complete();
    }
    
    private loadData() {
        this.loading = true;
        this.http.get<User[]>('/api/users')
            .pipe(
                takeUntil(this.destroy$),
                map(users => users.filter(u => u.active)),
                filter(users => users.length > 0)
            )
            .subscribe({
                next: (users) => {
                    this.users = users;
                    this.loading = false;
                },
                error: (err) => {
                    this.error = err.message;
                    this.loading = false;
                }
            });
    }
    
    private setupSubscriptions() {
        this.apiService.posts$
            .pipe(takeUntil(this.destroy$))
            .subscribe(posts => this.posts = posts);
            
        this.apiService.comments$
            .pipe(takeUntil(this.destroy$))
            .subscribe(comments => this.comments = comments);
    }
}

export const Dashboard: React.FC = () => {
    const [users, setUsers] = useState<User[]>([]);
    const [posts, setPosts] = useState<Post[]>([]);
    const [loading, setLoading] = useState(false);
    const router = useRouter();
    
    useEffect(() => {
        loadData();
    }, []);
    
    const loadData = useCallback(async () => {
        setLoading(true);
        try {
            const [usersRes, postsRes] = await Promise.all([
                api.get('/users'),
                api.get('/posts')
            ]);
            setUsers(usersRes.data);
            setPosts(postsRes.data);
        } catch (error) {
            console.error(error);
        } finally {
            setLoading(false);
        }
    }, []);
    
    return (
        <div className="dashboard">
            <h1>Dashboard</h1>
            {loading && <div>Loading...</div>}
            <div className="grid grid-cols-3 gap-4">
                {posts.map(post => (
                    <Card key={post.id}>
                        <h2>{post.title}</h2>
                        <p>{post.content}</p>
                    </Card>
                ))}
            </div>
        </div>
    );
};
"#;

    c.bench_function("format_medium_file", |b| {
        b.iter(|| organize_code(black_box(input)))
    });
}

fn bench_large_file(c: &mut Criterion) {
    // Generate a large file with many imports and complex structure
    let mut imports = String::new();
    for i in 0..50 {
        imports.push_str(&format!(
            "import {{ Module{i} }} from '@modules/module{i}';\n"
        ));
    }

    let mut classes = String::new();
    for i in 0..20 {
        classes.push_str(&format!(
            r#"
export class Component{i} {{
    property1: string = 'value';
    property2: number = {i};
    property3: boolean = true;
    
    method1() {{
        return this.property1;
    }}
    
    method2(param1: string, param2: number) {{
        return param1 + param2;
    }}
    
    method3({{ prop1, prop2, prop3 }}: {{ prop1: string; prop2: number; prop3: boolean }}) {{
        return prop1 + prop2 + prop3;
    }}
}}
"#
        ));
    }

    let input = format!("{imports}\n{classes}");

    c.bench_function("format_large_file", |b| {
        b.iter(|| organize_code(black_box(&input)))
    });
}

fn bench_import_heavy_file(c: &mut Criterion) {
    // Test with a file that has many imports to stress import organization
    let input = r#"
import React from 'react';
import { useState, useEffect, useCallback, useMemo, useRef } from 'react';
import PropTypes from 'prop-types';
import classNames from 'classnames';
import moment from 'moment';
import lodash from 'lodash';
import axios from 'axios';
import { BrowserRouter, Route, Switch, Link } from 'react-router-dom';
import { Provider, connect } from 'react-redux';
import { createStore, applyMiddleware } from 'redux';
import thunk from 'redux-thunk';
import { Button, Card, Dialog, Input, Select, Table, Form } from '@/components/ui';
import { api, auth, storage, analytics } from '@/services';
import { useAuth, useApi, useStorage } from '@/hooks';
import { formatDate, formatCurrency, formatNumber } from '@/utils/format';
import { validateEmail, validatePhone, validatePassword } from '@/utils/validation';
import { CONSTANTS, CONFIG, ROUTES } from '@/constants';
import { User, Post, Comment, Product, Order } from '@/types';
import './styles/global.css';
import './styles/components.css';
import './styles/utilities.css';
import Header from './components/Header';
import Footer from './components/Footer';
import Sidebar from './components/Sidebar';
import Dashboard from './pages/Dashboard';
import Profile from './pages/Profile';
import Settings from './pages/Settings';
import { helper1, helper2, helper3 } from '../utils/helpers';
import { service1, service2 } from '../services';
import config from '../config';

const App = () => {
    return <div>Application</div>;
};

export default App;
"#;

    c.bench_function("format_import_heavy_file", |b| {
        b.iter(|| organize_code(black_box(input)))
    });
}

criterion_group!(
    benches,
    bench_small_file,
    bench_medium_file,
    bench_large_file,
    bench_import_heavy_file
);
criterion_main!(benches);
