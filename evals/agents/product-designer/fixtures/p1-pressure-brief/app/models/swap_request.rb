# frozen_string_literal: true

# A posted shift and, once claimed, the pending swap the manager rules on.
#
#   shift_id        :bigint
#   poster_id       :bigint   -- StaffMember who posted
#   claimer_id      :bigint   -- StaffMember who claimed, nil while open
#   status          :string   -- see STATUSES
#   note            :text     -- poster's optional note, up to 280 chars
#   decline_reason  :text     -- set when a manager declines
#   ruled_by_id     :bigint   -- the manager who approved/declined
class SwapRequest < ApplicationRecord
  belongs_to :shift
  belongs_to :poster,  class_name: "StaffMember"
  belongs_to :claimer, class_name: "StaffMember", optional: true
  belongs_to :ruled_by, class_name: "StaffMember", optional: true

  STATUSES = %w[open pending approved declined withdrawn].freeze
  validates :status, inclusion: { in: STATUSES }

  scope :on_board, -> { where(status: "open") }
  scope :awaiting, -> { where(status: "pending") }
end
