import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import Settings from '~view/Settings';

describe('Settings', () => {
  it('renders without crashing', () => {
    render(<Settings />);
  });

  it('displays the Settings text', () => {
    render(<Settings />);
    expect(screen.getByText('Settings')).toBeInTheDocument();
  });
});
