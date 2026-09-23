# Development seed — a realistic week for the Clifton Village store.
#
# Numbers from the live database, September 2026:
#   9 stores · 130 staff · ~62 published shifts per store per week
#   swap board: typically 6–12 open posts chain-wide, 15+ in the week before a bank holiday
#   pending swaps awaiting a manager on a Monday morning: 3–7 per store

stores = [
  "Crumb & Co. Clifton Village", "Crumb & Co. Gloucester Road", "Crumb & Co. Bedminster",
  "Crumb & Co. Wapping Wharf", "Crumb & Co. Bath — Walcot Street", "Crumb & Co. Bath — Kingsmead",
  "Crumb & Co. Southville", "Crumb & Co. Redland", "Crumb & Co. Temple Meads Kiosk"
].map { |n| Store.find_or_create_by!(name: n) }

clifton = stores.first
staff = [
  ["Oluwaseun Adebayo-Whitfield", "supervisor"], ["Mei-Ling Tsang", "barista"],
  ["Bartłomiej Wróblewski", "baker"], ["Aoife Ní Bhriain", "barista"],
  ["Jo Smith", "barista"], ["Krishnamurthy Venkataraghavan", "keyholder"],
  ["Sara Al-Rashid", "barista"], ["Tomasz Nowak", "baker"], ["Leah O'Connor", "barista"],
  ["Daniel Kim", "barista"], ["Fatima Zahra El Idrissi", "supervisor"], ["Ben Ward", "barista"],
  ["Chidinma Okonkwo-Adeyemi", "barista"], ["Yusuf Demir", "baker"]
].map { |name, role| StaffMember.find_or_create_by!(name: name, store: clifton, default_role: role) }

week = Date.current.next_week
# Bakers start at 04:30; the Friday keyholder shift runs 18:00–01:30 across midnight.
shifts = []
7.times do |d|
  day = week + d
  shifts << Shift.create!(store: clifton, staff: staff[2], role: "baker",   starts_at: day.to_time.change(hour: 4, min: 30), ends_at: day.to_time.change(hour: 12))
  shifts << Shift.create!(store: clifton, staff: staff[1], role: "barista", starts_at: day.to_time.change(hour: 7), ends_at: day.to_time.change(hour: 15))
  shifts << Shift.create!(store: clifton, staff: staff[3], role: "barista", starts_at: day.to_time.change(hour: 11), ends_at: day.to_time.change(hour: 19))
end
shifts << Shift.create!(store: clifton, staff: staff[5], role: "keyholder", starts_at: (week + 4).to_time.change(hour: 18), ends_at: (week + 5).to_time.change(hour: 1, min: 30))
shifts.each { |s| s.update!(published_at: Time.current) }

# The board as it looks on a normal Monday: eight open posts, three pending.
SwapRequest.create!(shift: shifts[1],  poster: staff[1], status: "open",    note: "Dentist, can swap for any Thursday")
SwapRequest.create!(shift: shifts[4],  poster: staff[1], status: "open",    note: "")
SwapRequest.create!(shift: shifts[8],  poster: staff[3], status: "open",    note: "Happy to take an early in return — my daughter's nativity is at 2pm and I've promised her I'll be in the front row, so anything that finishes by 1 works. Could also do the Sunday open if that helps; message me on the group or grab me on Tuesday, I'm in from 7 till 3 and happy to.")
SwapRequest.create!(shift: shifts[9],  poster: staff[2], status: "open",    note: "4:30 start, sorry")
SwapRequest.create!(shift: shifts[21], poster: staff[5], status: "open",    note: "Overnight-ish, closes at half one")
SwapRequest.create!(shift: shifts[12], poster: staff[3], status: "open",    note: "")
SwapRequest.create!(shift: shifts[15], poster: staff[1], status: "open",    note: "")
SwapRequest.create!(shift: shifts[18], poster: staff[3], status: "open",    note: "Can't do Sundays this month")
SwapRequest.create!(shift: shifts[2],  poster: staff[3], claimer: staff[6], status: "pending")
SwapRequest.create!(shift: shifts[6],  poster: staff[2], claimer: staff[7], status: "pending")
SwapRequest.create!(shift: shifts[11], poster: staff[1], claimer: staff[9], status: "pending")
SwapRequest.create!(shift: shifts[5],  poster: staff[3], claimer: staff[11], status: "declined", decline_reason: "Ben would be on 46 hours this week; Mei-Ling's Tuesday post is still open on the board", ruled_by: staff[0])
