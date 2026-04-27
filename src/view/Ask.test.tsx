import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, act } from '@testing-library/react';

const { mockInvoke } = vi.hoisted(() => ({ mockInvoke: vi.fn() }));

vi.mock('@tauri-apps/api/core', () => ({ invoke: mockInvoke }));
vi.mock('react-hotkeys-hook', () => ({ useHotkeys: vi.fn() }));
vi.mock('~hooks/useInfo', () => ({ default: () => ({ platform: 'linux', isMac: false }) }));

import Ask from '~view/Ask';

describe('Ask (ChatInput)', () => {
  beforeEach(() => {
    mockInvoke.mockReset();
    mockInvoke.mockResolvedValue(undefined);
  });

  it('renders a textarea', () => {
    render(<Ask />);
    expect(screen.getByRole('textbox')).toBeInTheDocument();
  });

  it('renders the send button', () => {
    render(<Ask />);
    expect(screen.getByLabelText('Send message')).toBeInTheDocument();
  });

  it('shows placeholder text', () => {
    render(<Ask />);
    expect(screen.getByPlaceholderText('Type your message here...')).toBeInTheDocument();
  });

  it('does not call ask_send when message is empty', async () => {
    render(<Ask />);
    const sendBtn = screen.getByLabelText('Send message');
    await act(async () => {
      fireEvent.click(sendBtn);
    });
    expect(mockInvoke).not.toHaveBeenCalledWith('ask_send', expect.anything());
  });

  it('calls ask_send when send button is clicked with a non-empty message', async () => {
    render(<Ask />);
    const textarea = screen.getByRole('textbox');
    const sendBtn = screen.getByLabelText('Send message');

    await act(async () => {
      fireEvent.change(textarea, { target: { value: 'Hello world' } });
    });
    await act(async () => {
      fireEvent.click(sendBtn);
    });

    expect(mockInvoke).toHaveBeenCalledWith('ask_send', expect.anything());
  });

  it('clears the textarea after sending', async () => {
    render(<Ask />);
    const textarea = screen.getByRole('textbox');
    const sendBtn = screen.getByLabelText('Send message');

    await act(async () => {
      fireEvent.change(textarea, { target: { value: 'Test message' } });
    });
    await act(async () => {
      fireEvent.click(sendBtn);
    });

    expect((textarea as HTMLTextAreaElement).value).toBe('');
  });

  it('syncs message via ask_sync while typing (debounced)', async () => {
    vi.useFakeTimers();
    render(<Ask />);
    const textarea = screen.getByRole('textbox');

    await act(async () => {
      fireEvent.change(textarea, { target: { value: 'typing...' } });
    });

    await act(async () => {
      vi.advanceTimersByTime(400);
    });

    expect(mockInvoke).toHaveBeenCalledWith('ask_sync', expect.objectContaining({ message: expect.any(String) }));
    vi.useRealTimers();
  });
});

describe('Ask (ChatInput)', () => {
  beforeEach(() => {
    mockInvoke.mockReset();
    mockInvoke.mockResolvedValue(undefined);
  });

  it('renders a textarea', () => {
    render(<Ask />);
    expect(screen.getByRole('textbox')).toBeInTheDocument();
  });

  it('renders the send button', () => {
    render(<Ask />);
    expect(screen.getByLabelText('Send message')).toBeInTheDocument();
  });

  it('shows placeholder text', () => {
    render(<Ask />);
    expect(screen.getByPlaceholderText('Type your message here...')).toBeInTheDocument();
  });

  it('does not call ask_send when message is empty', async () => {
    render(<Ask />);
    const sendBtn = screen.getByLabelText('Send message');
    await act(async () => {
      fireEvent.click(sendBtn);
    });
    expect(mockInvoke).not.toHaveBeenCalledWith('ask_send', expect.anything());
  });

  it('calls ask_send when send button is clicked with a non-empty message', async () => {
    render(<Ask />);
    const textarea = screen.getByRole('textbox');
    const sendBtn = screen.getByLabelText('Send message');

    await act(async () => {
      fireEvent.change(textarea, { target: { value: 'Hello world' } });
    });
    await act(async () => {
      fireEvent.click(sendBtn);
    });

    expect(mockInvoke).toHaveBeenCalledWith('ask_send', expect.anything());
  });

  it('clears the textarea after sending', async () => {
    render(<Ask />);
    const textarea = screen.getByRole('textbox');
    const sendBtn = screen.getByLabelText('Send message');

    await act(async () => {
      fireEvent.change(textarea, { target: { value: 'Test message' } });
    });
    await act(async () => {
      fireEvent.click(sendBtn);
    });

    expect((textarea as HTMLTextAreaElement).value).toBe('');
  });

  it('syncs message via ask_sync while typing (debounced)', async () => {
    vi.useFakeTimers();
    render(<Ask />);
    const textarea = screen.getByRole('textbox');

    await act(async () => {
      fireEvent.change(textarea, { target: { value: 'typing...' } });
    });

    await act(async () => {
      vi.advanceTimersByTime(400);
    });

    expect(mockInvoke).toHaveBeenCalledWith('ask_sync', expect.objectContaining({ message: expect.any(String) }));
    vi.useRealTimers();
  });
});
