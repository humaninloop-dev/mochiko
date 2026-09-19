// FEAT-034 — first cut, not yet wired. Started from contracts/api.yaml before the review.
import { Injectable } from '@nestjs/common';

type Tier = 'full' | 'half' | 'none';

@Injectable()
export class RefundService {
  tierFor(noticeDays: number): Tier {
    if (noticeDays >= 14) return 'full';
    if (noticeDays >= 7) return 'half';
    return 'none';
  }

  async cancel(bookingId: string, actorId: string) {
    // TODO: load booking, compute noticeDays from startsOn, create Refund row,
    // issue credit note per D-004, enqueue outbox row per D-005
    throw new Error('not implemented');
  }
}
