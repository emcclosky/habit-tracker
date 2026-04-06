export interface Habit {
  name: string;
  completions: string[]; // ISO date string
  streak: number;
}

export type HabitResponse = Habit[];

export interface CreateHabitRequest {
  name: string;
}

export interface ApiErrorResponse {
  message: string;
}
