// apps/web-dashboard/app/api/github/runs/route.ts
import { NextRequest, NextResponse } from 'next/server';
import { auth } from '@/lib/auth/config';

const GITHUB_PAT = process.env.GITHUB_PAT;
const GITHUB_OWNER = process.env.GITHUB_OWNER;
const GITHUB_REPO = process.env.GITHUB_REPO;
const WORKFLOW_ID = 'provisioner-cron.yml';

export async function GET(req: NextRequest) {
  const session = await auth();
  if (!session) return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });

  const url = `https://api.github.com/repos/${GITHUB_OWNER}/${GITHUB_REPO}/actions/workflows/${WORKFLOW_ID}/runs?per_page=5`;

  try {
    const response = await fetch(url, {
      headers: {
        'Authorization': `Bearer ${GITHUB_PAT}`,
        'Accept': 'application/vnd.github.v3+json',
      },
      next: { revalidate: 10 } // Cache corto
    });

    if (!response.ok) throw new Error('GitHub API Unreachable');

    const data = await response.json();
    return NextResponse.json(data.workflow_runs); // Retorna array de WorkflowRun
  } catch (e) {
    return NextResponse.json({ error: 'Failed to fetch runs' }, { status: 502 });
  }
}
