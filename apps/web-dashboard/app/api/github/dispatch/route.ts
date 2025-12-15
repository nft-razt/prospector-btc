// apps/web-dashboard/app/api/github/dispatch/route.ts
/**
 * =================================================================
 * APARATO: GITHUB DISPATCHER (SERVER PROXY)
 * RESPONSABILIDAD: TRIGGER REMOTO DE WORKFLOWS (SEGURIDAD DE TOKEN)
 * =================================================================
 */

import { NextRequest, NextResponse } from "next/server";
import { z } from "zod";
import { auth } from "@/lib/auth/config";
import { SwarmLaunchSchema } from "@prospector/api-contracts";

// Validación de entorno en tiempo de ejecución
const GITHUB_PAT = process.env.GITHUB_PAT;
const GITHUB_OWNER = process.env.GITHUB_OWNER;
const GITHUB_REPO = process.env.GITHUB_REPO;
const WORKFLOW_ID = "provisioner-cron.yml"; // Nombre del archivo en .github/workflows

export async function POST(req: NextRequest) {
  // 1. Security Checkpoint (Solo admins logueados)
  const session = await auth();
  if (!session) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }

  if (!GITHUB_PAT || !GITHUB_OWNER || !GITHUB_REPO) {
    return NextResponse.json(
      { error: "Server Misconfiguration: Missing GitHub Credentials" },
      { status: 500 },
    );
  }

  try {
    // 2. Validación de Payload
    const body = await req.json();
    const config = SwarmLaunchSchema.parse(body);

    // 3. Llamada a GitHub API (Dispatch Event)
    // Docs: https://docs.github.com/en/rest/actions/workflows#create-a-workflow-dispatch-event
    const url = `https://api.github.com/repos/${GITHUB_OWNER}/${GITHUB_REPO}/actions/workflows/${WORKFLOW_ID}/dispatches`;

    const response = await fetch(url, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${GITHUB_PAT}`,
        Accept: "application/vnd.github.v3+json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        ref: config.ref,
        inputs: {
          worker_count_per_shard: config.worker_count.toString(),
          shard_count: config.shard_count.toString(),
        },
      }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      console.error("GitHub API Error:", errorText);
      return NextResponse.json(
        { error: "Failed to trigger workflow", details: errorText },
        { status: response.status },
      );
    }

    return NextResponse.json({
      success: true,
      message: "Swarm sequence initiated",
    });
  } catch (error) {
    if (error instanceof z.ZodError) {
      return NextResponse.json(
        { error: "Invalid Configuration", details: error.errors },
        { status: 400 },
      );
    }
    return NextResponse.json(
      { error: "Internal Server Error" },
      { status: 500 },
    );
  }
}
