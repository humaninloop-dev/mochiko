# frozen_string_literal: true

# A published slot on a store's rota.
#
#   store_id      :bigint    -- Store (nine of them; names up to 38 chars, e.g. "Crumb & Co. Clifton Village")
#   staff_id      :bigint    -- StaffMember
#   role          :string    -- "barista" | "baker" | "supervisor" | "keyholder"
#   starts_at     :datetime  -- shifts can start at 04:30 (bakers) and can cross midnight
#   ends_at       :datetime
#   published_at  :datetime  -- nil until the manager publishes the week
class Shift < ApplicationRecord
  belongs_to :store
  belongs_to :staff, class_name: "StaffMember"
  has_many :swap_requests, dependent: :destroy

  ROLES = %w[barista baker supervisor keyholder].freeze

  scope :upcoming,  -> { where("starts_at > ?", Time.current) }
  scope :published, -> { where.not(published_at: nil) }

  def overlaps?(other)
    starts_at < other.ends_at && other.starts_at < ends_at
  end

  def duration_hours
    ((ends_at - starts_at) / 1.hour).round(2)
  end
end
