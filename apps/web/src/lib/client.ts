import type { ApiErrorResponse } from "@habit-tracker/shared";

class HttpError extends Error {
	constructor(
		public status: number,
		message: string,
	) {
		super(message);
		this.name = "HttpError";
	}
}

export async function apiFetch<T>(
	path: string,
	options?: RequestInit,
): Promise<T> {
	const response = await fetch(`${process.env.API_URL}${path}`, {
		...options,
		headers: {
			"Content-Type": "application/json",
			...options?.headers,
		},
	});
	if (!response.ok) {
		const body: ApiErrorResponse = await response.json();
		throw new HttpError(
			response.status,
			body.message || "An error occurred while fetching data",
		);
	}

	return (await response.json()) as T;
}
