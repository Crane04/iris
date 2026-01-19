
export interface MatchResult {
  name: string;
  probability: number;
}

export interface ComparisonResponse {
  request_id: string;
  status: string;
  matches: MatchResult[];
  execution_time: string;
  stateless: boolean;
}

export interface WorkflowStep {
  id: string;
  title: string;
  icon: React.ReactNode;
  code?: string;
  subtitle?: string;
  description: string;
}
