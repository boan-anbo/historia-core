export interface Event {
  meta: Meta;
  raw_strings: RawStrings;
  start_year: EventDate | null;
  start_month: EventDate | null;
  start_day: EventDate | null;
  is_start_decade: boolean | null;
  is_start_century: boolean | null;
  start_date: string | null;
}

export interface RawStrings {
  raw_event_string: string;
  raw_context_string: string;
}

export interface EventDate {
  number_string_raw: string;
  number_string_arabic: string;
  number: number;
  is_inferred: boolean;
  inferred_from: string;
}

export interface Meta {
  id: string;
  created: string;
}