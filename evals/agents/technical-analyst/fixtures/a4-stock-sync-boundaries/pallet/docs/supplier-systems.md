# Suppliers' inventory systems — what we know (ops survey, August 2026)

| System | Suppliers | How stock is available to us |
|---|---|---|
| Lightspeed Retail (X-Series) | 38 | REST API, polling only. OAuth 2 per supplier: the access token lives 24 h and the refresh token rotates on every use (a lost rotation means the supplier reconnects by hand). `GET /api/2.0/inventory` paginated at 250 rows. Rate limit 60 requests per minute per token; over it, `429` with `Retry-After`. Sandbox available. |
| Shopify | 21 | Webhooks: `inventory_levels/update` pushed to a URL we register per shop, HMAC-SHA256 signed with the shop's secret. Shopify retries a failed delivery 19 times over 48 h and deliveries can arrive out of order; the payload carries `updated_at`. No history or bulk endpoint on the plan most of these suppliers are on, so a missed event is gone. |
| Sage 50, via their wholesaler | 9 (all through one wholesaler, Northern Provisions) | A CSV dropped nightly around 02:00 on the wholesaler's SFTP host, one row per product code, from an export their IT runs. The file was missing on 4 of the last 30 nights and truncated twice. Their firewall allowlists our worker's egress IP; changing it needs 10 working days' notice to their IT (confirmed by e-mail, 2026-08-19). |
| None | 72 | Paper, or memory. |

Product matching: Lightspeed and Shopify carry a SKU we can store on `Product.supplier_sku`.
The Sage export uses Northern Provisions' internal codes, which do not match the SKUs the
nine suppliers print on their price lists; nobody has mapped them.
