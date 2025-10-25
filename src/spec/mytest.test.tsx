/// <reference lib="dom" />

import { test, expect } from 'bun:test';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';

function Button({ children }: { children: React.ReactNode }) {
    return <button>{children}</button>;
}

test('renders button', () => {
    render(<Button>Click me</Button>);
    expect(screen.getByRole('button')).toHaveTextContent('Click me');
});