# Development Patterns — Rails Stack

Ruby on Rails / Hotwire (Turbo + Stimulus) / Tailwind / PostgreSQL

This document captures stack-specific patterns, conventions, and decisions for Rails stack projects (Ruby on Rails with Hotwire, Turbo, Stimulus, Tailwind CSS, and PostgreSQL). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure Rails apps, test with Minitest/RSpec, use Hotwire, deploy with Kamal, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Ruby | 3.3+ | Managed via rbenv or asdf |
| Rails | 8.0+ | Full-stack framework |
| Turbo | 8.0+ | SPA-like navigation, frames, streams |
| Stimulus | 4.0+ | Modest JavaScript framework for HTML |
| Turbo Native | 1.0+ | Mobile apps sharing web views |
| Tailwind CSS | 4.x | Utility-first CSS via `tailwindcss-rails` |
| PostgreSQL | 16+ | Primary database |
| Solid Queue | 1.0+ | Rails 8 default job backend (DB-backed) |
| Solid Cache | 1.0+ | Rails 8 default cache backend (DB-backed) |
| Solid Cable | 1.0+ | Rails 8 default Action Cable backend (DB-backed) |
| Propshaft | 1.0+ | Rails 8 default asset pipeline |
| Kamal | 2.0+ | Rails 8 default deployment tool |
| Node.js | 22+ | Optional — only if using jsbundling/cssbundling |
| Redis | 7+ | Optional — only if using Sidekiq or non-Solid backends |

### Version Constraint Policy

Use pessimistic version constraints in `Gemfile` pinned to the minor version:

```ruby
# Good — allows patch updates, blocks minor/major
gem "rails", "~> 8.0.0"
gem "turbo-rails", "~> 2.0"
gem "stimulus-rails", "~> 1.3"
gem "solid_queue", "~> 1.0"

# Bad — too loose, allows breaking minor updates
gem "rails", "~> 8.0"

# Bad — too tight, blocks patch fixes
gem "rails", "8.0.1"
```

Exception: for release candidates or gems with known instability, pin exact with `gem "foo", "1.2.3"`.

### Rails 8 Defaults

Rails 8 changed several defaults. Know what ships out of the box:

| Feature | Rails 7 Default | Rails 8 Default |
|---|---|---|
| Job backend | None (configure yourself) | Solid Queue |
| Cache backend | Redis/Memcached | Solid Cache |
| WebSocket backend | Redis | Solid Cable |
| Asset pipeline | Sprockets | Propshaft |
| Deployment | Capistrano/custom | Kamal |
| Authentication | None | Built-in generator (`bin/rails generate authentication`) |
| CSS | Import maps + Tailwind | Tailwind via `tailwindcss-rails` |
| JavaScript | Import maps | Import maps |

---

## 2. Project Structure

### Standard Rails Convention

Rails follows convention over configuration. The directory structure is well-defined:

```
app/
├── assets/                  # Stylesheets, images
│   ├── stylesheets/
│   │   └── application.tailwind.css
│   └── images/
├── channels/                # Action Cable channels
│   └── application_cable/
│       ├── channel.rb
│       └── connection.rb
├── controllers/             # Request handlers
│   ├── application_controller.rb
│   ├── concerns/            # Controller concerns (shared behavior)
│   │   ├── authentication.rb
│   │   └── set_current_request_details.rb
│   ├── sessions_controller.rb
│   └── api/                 # API namespace
│       └── v1/
│           └── base_controller.rb
├── helpers/                 # View helpers (prefer view components)
├── javascript/              # Stimulus controllers, Turbo config
│   ├── application.js
│   └── controllers/
│       ├── index.js
│       ├── hello_controller.js
│       └── dropdown_controller.js
├── jobs/                    # Active Job classes
│   ├── application_job.rb
│   └── sync_contact_job.rb
├── mailers/                 # Action Mailer classes
│   └── application_mailer.rb
├── models/                  # ActiveRecord models
│   ├── application_record.rb
│   ├── concerns/            # Model concerns (shared behavior)
│   │   ├── sluggable.rb
│   │   └── archivable.rb
│   ├── user.rb
│   ├── post.rb
│   └── comment.rb
├── services/                # Service objects (not Rails default)
│   ├── base_service.rb
│   └── stripe/
│       ├── create_customer.rb
│       └── process_webhook.rb
├── components/              # ViewComponent classes
│   ├── button_component.rb
│   ├── card_component.rb
│   └── modal_component.rb
├── views/                   # ERB/HTML templates
│   ├── layouts/
│   │   ├── application.html.erb
│   │   └── mailer.html.erb
│   ├── shared/              # Shared partials
│   │   ├── _navbar.html.erb
│   │   ├── _flash.html.erb
│   │   └── _footer.html.erb
│   └── posts/
│       ├── index.html.erb
│       ├── show.html.erb
│       ├── _form.html.erb
│       └── _post.html.erb
├── lib/                     # Non-Rails Ruby code
│   └── middleware/
├── config/                  # Configuration
│   ├── application.rb
│   ├── database.yml
│   ├── environments/
│   │   ├── development.rb
│   │   ├── test.rb
│   │   └── production.rb
│   ├── initializers/
│   │   ├── filter_parameter_logging.rb
│   │   └── content_security_policy.rb
│   └── routes.rb
├── db/
│   ├── migrate/             # Database migrations
│   ├── schema.rb            # Auto-generated schema snapshot
│   └── seeds.rb             # Seed data
└── test/ (or spec/)         # Tests
```

### Service Object Convention

Rails does not prescribe a service layer, but complex business logic belongs in service objects, not models or controllers:

```ruby
# app/services/base_service.rb
class BaseService
  def self.call(...)
    new(...).call
  end

  def initialize(...)
    # accept dependencies
  end

  def call
    raise NotImplementedError
  end
end

# app/services/stripe/create_customer.rb
module Stripe
  class CreateCustomer < BaseService
    def initialize(user:, client: Stripe::Client)
      @user = user
      @client = client
    end

    def call
      customer = @client.customers.create(
        email: @user.email,
        name: @user.name
      )
      @user.update!(stripe_customer_id: customer.id)
      customer
    end
  end
end
```

**Convention:** Service objects go in `app/services/`. Use `self.call` class method pattern. Accept dependencies via constructor for testability. Return the result, raise on failure, or use a Result object pattern.

### Concerns Convention

Concerns extract shared behavior from models and controllers:

```ruby
# app/models/concerns/sluggable.rb
module Sluggable
  extend ActiveSupport::Concern

  included do
    before_validation :generate_slug, on: :create
    validates :slug, presence: true, uniqueness: true
  end

  private

  def generate_slug
    self.slug ||= name&.parameterize
  end
end

# app/models/post.rb
class Post < ApplicationRecord
  include Sluggable
end
```

**Convention:** Use concerns for shared behavior that makes sense across multiple models or controllers. Do NOT use concerns to hide complexity — a 500-line concern is worse than a 500-line model. If the concern only applies to one model, it should stay in the model.

---

## 3. ActiveRecord Patterns

### Migration Template

Every migration is reversible and uses strong typing:

```ruby
class CreatePosts < ActiveRecord::Migration[8.0]
  def change
    create_table :posts, id: :uuid do |t|
      t.references :user, null: false, foreign_key: true, type: :uuid
      t.string :title, null: false
      t.string :slug, null: false
      t.text :body, null: false
      t.integer :status, null: false, default: 0
      t.datetime :published_at
      t.timestamps
    end

    add_index :posts, :slug, unique: true
    add_index :posts, [:user_id, :status]
    add_index :posts, :published_at
  end
end
```

**Conventions:**
- Always use `id: :uuid` for primary keys (configure in `ApplicationRecord`)
- Always add `null: false` where the column is required
- Always add `foreign_key: true` on references
- Always add indexes for columns used in queries, foreign keys, and unique constraints
- Never edit a migration after it has been merged. Write a new corrective migration instead
- Use `change` (reversible) over `up`/`down` when possible
- Name migrations descriptively: `AddPublishedAtToPosts`, `CreateUserPreferences`

### UUID Primary Keys

Configure UUID as the default primary key type:

```ruby
# app/models/application_record.rb
class ApplicationRecord < ActiveRecord::Base
  primary_abstract_class
  self.implicit_order_column = "created_at"
end

# config/initializers/generators.rb
Rails.application.config.generators do |g|
  g.orm :active_record, primary_key_type: :uuid
end

# db/migrate/000_enable_pgcrypto.rb
class EnablePgcrypto < ActiveRecord::Migration[8.0]
  def change
    enable_extension "pgcrypto"
  end
end
```

### Association Patterns

```ruby
class User < ApplicationRecord
  # Always specify dependent behavior
  has_many :posts, dependent: :destroy
  has_many :comments, dependent: :destroy
  has_one :profile, dependent: :destroy

  # Use inverse_of for performance (avoids extra queries)
  has_many :memberships, inverse_of: :user
  has_many :teams, through: :memberships

  # Scoped associations
  has_many :published_posts, -> { published }, class_name: "Post"

  # Strict loading — catch N+1 at development time
  self.strict_loading_by_default = true
end

class Post < ApplicationRecord
  belongs_to :user
  belongs_to :category, optional: true  # explicit about nullable FK
  has_many :comments, dependent: :destroy
  has_many_attached :images  # Active Storage
  has_rich_text :body        # Action Text
end
```

### Scopes

Scopes are named queries. Use them instead of class methods for chainability:

```ruby
class Post < ApplicationRecord
  # Enum for status
  enum :status, { draft: 0, published: 1, archived: 2 }

  # Scopes — always use lambdas
  scope :recent, -> { order(created_at: :desc) }
  scope :published, -> { where(status: :published) }
  scope :by_author, ->(user) { where(user: user) }
  scope :published_since, ->(date) { published.where("published_at >= ?", date) }
  scope :search, ->(query) {
    where("title ILIKE :q OR body ILIKE :q", q: "%#{sanitize_sql_like(query)}%")
  }

  # Scopes with preloads
  scope :with_author, -> { includes(:user) }
  scope :with_comments_count, -> { left_joins(:comments).select("posts.*, COUNT(comments.id) AS comments_count").group(:id) }
end

# Usage — chainable
Post.published.recent.with_author.limit(10)
Post.by_author(current_user).search("rails").page(params[:page])
```

### Callbacks — Use Sparingly

Callbacks create hidden coupling. Prefer service objects for complex side effects:

```ruby
class Post < ApplicationRecord
  # GOOD — simple data normalization
  before_validation :normalize_title
  before_save :generate_slug, if: -> { title_changed? }

  # GOOD — counter cache maintenance
  after_create :increment_posts_count
  after_destroy :decrement_posts_count

  # BAD — external side effects in callbacks
  # after_create :send_notification_email     # Use a service or job instead
  # after_save :sync_to_search_engine         # Use a job instead
  # after_destroy :delete_from_stripe         # Use a service instead

  private

  def normalize_title
    self.title = title&.strip&.squeeze(" ")
  end

  def generate_slug
    self.slug = title.parameterize
  end
end
```

**Rule:** Callbacks should only modify the record itself or its direct database state. Any external side effect (email, API call, job enqueue) belongs in a service object or controller.

### Single Table Inheritance (STI)

Use STI when subclasses share the same columns and table:

```ruby
# db/migrate — single table with type column
create_table :notifications, id: :uuid do |t|
  t.string :type, null: false  # Rails STI column
  t.references :user, null: false, foreign_key: true, type: :uuid
  t.references :notifiable, polymorphic: true, null: false, type: :uuid
  t.string :message, null: false
  t.datetime :read_at
  t.timestamps
end

# app/models/notification.rb
class Notification < ApplicationRecord
  belongs_to :user
  belongs_to :notifiable, polymorphic: true
end

# app/models/email_notification.rb
class EmailNotification < Notification
  def deliver
    NotificationMailer.notify(user, message).deliver_later
  end
end

# app/models/sms_notification.rb
class SmsNotification < Notification
  def deliver
    SmsService.call(user: user, message: message)
  end
end
```

**When to use STI:** Subclasses share 90%+ of columns. If subclasses have many unique columns, use delegated types instead.

### Delegated Types

Delegated types (Rails 6.1+) are preferred over polymorphic associations when the subtypes have different data:

```ruby
# app/models/entry.rb — the delegator
class Entry < ApplicationRecord
  delegated_type :entryable, types: %w[Message Comment Photo], dependent: :destroy
  belongs_to :user

  # Shared behavior
  scope :recent, -> { order(created_at: :desc) }
end

# app/models/message.rb — a delegate
class Message < ApplicationRecord
  has_one :entry, as: :entryable, touch: true
  validates :body, presence: true
end

# app/models/photo.rb — a delegate
class Photo < ApplicationRecord
  has_one :entry, as: :entryable, touch: true
  has_one_attached :image
  validates :caption, presence: true
end

# Usage
Entry.recent.includes(:entryable)
entry.entryable  # => #<Message> or #<Photo>
entry.message?   # => true/false
```

### Strict Loading

Strict loading prevents N+1 queries by raising when an association is lazily loaded:

```ruby
# Per-model default (development + test)
class ApplicationRecord < ActiveRecord::Base
  self.strict_loading_by_default = Rails.env.local?
end

# Per-query
users = User.strict_loading.all
users.first.posts  # => raises ActiveRecord::StrictLoadingViolationError

# Eager load to satisfy strict loading
users = User.includes(:posts, :profile).strict_loading.all
users.first.posts  # => works, already loaded

# Per-association override
class User < ApplicationRecord
  has_many :audit_logs, strict_loading: false  # exempt from strict loading
end
```

### Validations

```ruby
class User < ApplicationRecord
  # Presence
  validates :name, presence: true
  validates :email, presence: true, uniqueness: { case_sensitive: false }

  # Format
  validates :email, format: { with: URI::MailTo::EMAIL_REGEXP }
  validates :slug, format: { with: /\A[a-z0-9\-]+\z/, message: "only lowercase letters, numbers, and hyphens" }

  # Numericality
  validates :age, numericality: { greater_than: 0, less_than: 150 }, allow_nil: true

  # Length
  validates :name, length: { minimum: 2, maximum: 100 }
  validates :bio, length: { maximum: 500 }

  # Custom
  validate :password_complexity, if: -> { password.present? }

  # Conditional
  validates :company_name, presence: true, if: :business_account?

  private

  def password_complexity
    return if password.match?(/\A(?=.*[a-z])(?=.*[A-Z])(?=.*\d).{8,}\z/)
    errors.add(:password, "must include uppercase, lowercase, and a number")
  end
end
```

**Convention:** Always back uniqueness validations with a database unique index. The validation alone is not sufficient due to race conditions.

---

## 4. Authentication

### Rails 8 Built-in Authentication Generator

Rails 8 ships a built-in authentication generator. Use it as the default for new projects:

```bash
bin/rails generate authentication
```

This generates:
- `User` model with `email_address` and `password_digest`
- `Session` model for session tracking
- `SessionsController` for login/logout
- `Authentication` concern for `Current.user` and `require_authentication`
- Password reset flow (mailer, controller, token)
- `bcrypt` gem dependency
- Database migrations for users and sessions

```ruby
# app/models/user.rb (generated)
class User < ApplicationRecord
  has_secure_password
  has_many :sessions, dependent: :destroy

  normalizes :email_address, with: ->(e) { e.strip.downcase }
  validates :email_address, presence: true, uniqueness: true
end

# app/controllers/concerns/authentication.rb (generated)
module Authentication
  extend ActiveSupport::Concern

  included do
    before_action :require_authentication
    helper_method :authenticated?
  end

  private

  def require_authentication
    resume_session || request_authentication
  end

  def authenticated?
    Current.session.present?
  end

  def request_authentication
    session[:return_to_after_authenticating] = request.url
    redirect_to new_session_path
  end
end
```

**Convention:** Use the Rails 8 generator for new projects. It provides a solid, minimal auth system without external dependencies.

### has_secure_password

The foundation of Rails authentication. Requires `bcrypt` gem:

```ruby
class User < ApplicationRecord
  has_secure_password

  # has_secure_password provides:
  # - password=     (setter that hashes into password_digest)
  # - authenticate   (verify password, returns user or false)
  # - password_confirmation= (optional confirmation check)
end

# Usage
user = User.create!(email: "test@example.com", password: "secret123", password_confirmation: "secret123")
user.authenticate("secret123")  # => user
user.authenticate("wrong")      # => false
```

### Devise (Existing Projects)

For projects already using Devise or needing OAuth, OmniAuth, two-factor, or other advanced features:

```ruby
# Gemfile
gem "devise", "~> 4.9"

# app/models/user.rb
class User < ApplicationRecord
  devise :database_authenticatable, :registerable,
         :recoverable, :rememberable, :validatable,
         :confirmable, :lockable, :trackable

  # Devise modules:
  # :database_authenticatable — email/password login
  # :registerable — sign up
  # :recoverable — password reset
  # :rememberable — "remember me" cookie
  # :validatable — email/password validations
  # :confirmable — email confirmation
  # :lockable — lock after N failed attempts
  # :trackable — sign-in count, timestamps, IPs
  # :omniauthable — OAuth via OmniAuth
  # :timeoutable — session timeout
end
```

**Convention:** For new Rails 8 projects, start with the built-in generator. Only add Devise if you need OAuth, two-factor, or features the generator does not provide.

### Role-Based Authorization

Use an enum or a dedicated authorization gem:

```ruby
# Simple enum approach (small apps)
class User < ApplicationRecord
  enum :role, { member: 0, admin: 1, super_admin: 2 }

  def can_manage?(resource)
    admin? || super_admin?
  end
end

# Pundit (recommended for policy-based authorization)
# Gemfile
gem "pundit", "~> 2.4"

# app/policies/application_policy.rb
class ApplicationPolicy
  attr_reader :user, :record

  def initialize(user, record)
    @user = user
    @record = record
  end

  def index?   = false
  def show?    = false
  def create?  = false
  def update?  = false
  def destroy? = false
end

# app/policies/post_policy.rb
class PostPolicy < ApplicationPolicy
  def show?
    true
  end

  def update?
    user.admin? || record.user_id == user.id
  end

  def destroy?
    user.admin? || record.user_id == user.id
  end

  class Scope < ApplicationPolicy::Scope
    def resolve
      if user.admin?
        scope.all
      else
        scope.where(user: user).or(scope.published)
      end
    end
  end
end

# In controller
class PostsController < ApplicationController
  def update
    @post = Post.find(params[:id])
    authorize @post  # raises Pundit::NotAuthorizedError if forbidden
    # ...
  end
end
```

---

## 5. Component Library

### ViewComponent (Recommended)

ViewComponent by GitHub is the standard for building reusable, testable UI components in Rails:

```ruby
# Gemfile
gem "view_component", "~> 3.0"

# app/components/button_component.rb
class ButtonComponent < ViewComponent::Base
  VARIANTS = {
    primary: "bg-blue-600 text-white hover:bg-blue-700",
    secondary: "bg-gray-200 text-gray-800 hover:bg-gray-300",
    danger: "bg-red-600 text-white hover:bg-red-700"
  }.freeze

  SIZES = {
    sm: "px-3 py-1.5 text-sm",
    md: "px-4 py-2 text-base",
    lg: "px-6 py-3 text-lg"
  }.freeze

  def initialize(variant: :primary, size: :md, disabled: false, **html_attrs)
    @variant = variant
    @size = size
    @disabled = disabled
    @html_attrs = html_attrs
  end

  def call
    content_tag :button,
      content,
      class: classes,
      disabled: @disabled,
      **@html_attrs
  end

  private

  def classes
    [
      "inline-flex items-center justify-center rounded-md font-medium",
      "transition-colors duration-150 focus:outline-none focus:ring-2 focus:ring-offset-2",
      VARIANTS.fetch(@variant),
      SIZES.fetch(@size),
      ("opacity-50 cursor-not-allowed" if @disabled)
    ].compact.join(" ")
  end
end

# Usage in ERB
<%= render ButtonComponent.new(variant: :primary, size: :lg) do %>
  Save Changes
<% end %>
```

### ViewComponent Testing

```ruby
# test/components/button_component_test.rb
class ButtonComponentTest < ViewComponent::TestCase
  def test_renders_primary_button
    render_inline(ButtonComponent.new(variant: :primary)) { "Click me" }

    assert_selector "button.bg-blue-600", text: "Click me"
  end

  def test_renders_disabled_state
    render_inline(ButtonComponent.new(disabled: true)) { "Disabled" }

    assert_selector "button[disabled]"
    assert_selector "button.opacity-50"
  end
end
```

### Phlex (Alternative)

Phlex is a Ruby-first view framework — pure Ruby instead of ERB:

```ruby
# Gemfile
gem "phlex-rails", "~> 2.0"

# app/views/components/card.rb
class Components::Card < Phlex::HTML
  def initialize(title:, subtitle: nil)
    @title = title
    @subtitle = subtitle
  end

  def view_template
    div(class: "bg-white rounded-lg shadow-md p-6") do
      h3(class: "text-lg font-semibold text-gray-900") { @title }
      p(class: "text-sm text-gray-500 mt-1") { @subtitle } if @subtitle
      div(class: "mt-4") { yield }
    end
  end
end

# Usage
render Components::Card.new(title: "Recent Activity", subtitle: "Last 7 days") do
  render Components::ActivityList.new(activities: @activities)
end
```

**Convention:** Pick one component system (ViewComponent or Phlex) and use it consistently. Do not mix. ViewComponent is more established with broader ecosystem support. Phlex is newer, faster, and more Ruby-idiomatic.

### Turbo Frame Components

Build components that leverage Turbo Frames for progressive enhancement:

```ruby
# app/components/inline_edit_component.rb
class InlineEditComponent < ViewComponent::Base
  def initialize(record:, field:, url:)
    @record = record
    @field = field
    @url = url
  end

  erb_template <<~ERB
    <turbo-frame id="<%= dom_id(@record, @field) %>">
      <div class="group flex items-center gap-2">
        <span><%= @record.public_send(@field) %></span>
        <a href="<%= @url %>"
           class="invisible group-hover:visible text-blue-600 text-sm"
           data-turbo-frame="<%= dom_id(@record, @field) %>">
          Edit
        </a>
      </div>
    </turbo-frame>
  ERB
end
```

### Stimulus Component Pattern

Pair ViewComponents with Stimulus controllers for interactive behavior:

```ruby
# app/components/dropdown_component.rb
class DropdownComponent < ViewComponent::Base
  def initialize(label:)
    @label = label
  end

  erb_template <<~ERB
    <div data-controller="dropdown" class="relative">
      <button data-action="click->dropdown#toggle"
              data-dropdown-target="button"
              class="btn btn-secondary">
        <%= @label %>
      </button>
      <div data-dropdown-target="menu"
           class="hidden absolute z-10 mt-2 w-48 bg-white rounded-md shadow-lg">
        <%= content %>
      </div>
    </div>
  ERB
end

# app/javascript/controllers/dropdown_controller.js
import { Controller } from "@hotwired/stimulus"

export default class extends Controller {
  static targets = ["menu", "button"]

  toggle() {
    this.menuTarget.classList.toggle("hidden")
  }

  // Close when clicking outside
  close(event) {
    if (!this.element.contains(event.target)) {
      this.menuTarget.classList.add("hidden")
    }
  }

  connect() {
    document.addEventListener("click", this.close.bind(this))
  }

  disconnect() {
    document.removeEventListener("click", this.close.bind(this))
  }
}
```

---

## 6. Testing Patterns

### Test Pyramid (Rails-specific)

```
        /\
       /  \          E2E (Playwright / Cypress) — critical paths only
      /    \
     /------\
    /        \        System Tests (Capybara + Selenium/Cuprite)
   /          \       Full browser interactions, JavaScript, Turbo
  /------------\
 /              \      Integration Tests (request specs / controller tests)
/                \     HTTP requests, routing, authentication, response codes
/------------------\
/                    \   Unit Tests (model specs / model tests)
/                      \  Validations, scopes, associations, service objects
/------------------------\
```

### Minitest (Rails Default)

Rails ships with Minitest. It is lighter, faster, and simpler than RSpec:

```ruby
# test/models/post_test.rb
require "test_helper"

class PostTest < ActiveSupport::TestCase
  test "valid post with all required attributes" do
    post = Post.new(title: "Hello", body: "World", user: users(:admin))
    assert post.valid?
  end

  test "invalid without title" do
    post = Post.new(body: "World", user: users(:admin))
    assert_not post.valid?
    assert_includes post.errors[:title], "can't be blank"
  end

  test "slug is generated from title" do
    post = Post.create!(title: "Hello World", body: "Content", user: users(:admin))
    assert_equal "hello-world", post.slug
  end

  test ".published scope returns only published posts" do
    assert_includes Post.published, posts(:published_post)
    assert_not_includes Post.published, posts(:draft_post)
  end
end
```

### RSpec (Alternative)

Many Rails teams prefer RSpec for its expressive DSL:

```ruby
# Gemfile (test group)
gem "rspec-rails", "~> 7.0"
gem "factory_bot_rails", "~> 6.4"
gem "shoulda-matchers", "~> 6.0"
gem "faker", "~> 3.0"

# spec/models/post_spec.rb
require "rails_helper"

RSpec.describe Post, type: :model do
  describe "validations" do
    it { is_expected.to validate_presence_of(:title) }
    it { is_expected.to validate_presence_of(:body) }
    it { is_expected.to validate_uniqueness_of(:slug) }
  end

  describe "associations" do
    it { is_expected.to belong_to(:user) }
    it { is_expected.to have_many(:comments).dependent(:destroy) }
  end

  describe ".published" do
    it "returns only published posts" do
      published = create(:post, status: :published)
      _draft = create(:post, status: :draft)

      expect(Post.published).to contain_exactly(published)
    end
  end

  describe "#generate_slug" do
    it "parameterizes the title" do
      post = create(:post, title: "Hello World")
      expect(post.slug).to eq("hello-world")
    end
  end
end
```

### Fixtures vs Factories

**Fixtures** (Minitest default) — YAML files loaded once into the test database:

```yaml
# test/fixtures/users.yml
admin:
  name: Admin User
  email_address: admin@example.com
  password_digest: <%= BCrypt::Password.create("password") %>
  role: admin

member:
  name: Regular Member
  email_address: member@example.com
  password_digest: <%= BCrypt::Password.create("password") %>
  role: member

# test/fixtures/posts.yml
published_post:
  title: Published Post
  slug: published-post
  body: Some content
  status: 1
  user: admin
  published_at: <%= 1.day.ago %>

draft_post:
  title: Draft Post
  slug: draft-post
  body: Draft content
  status: 0
  user: member
```

**Factories** (factory_bot) — programmatic object creation:

```ruby
# spec/factories/users.rb (or test/factories/users.rb)
FactoryBot.define do
  factory :user do
    name { Faker::Name.name }
    sequence(:email_address) { |n| "user#{n}@example.com" }
    password { "password123" }
    role { :member }

    trait :admin do
      role { :admin }
    end

    trait :with_posts do
      after(:create) do |user|
        create_list(:post, 3, user: user)
      end
    end
  end
end

# spec/factories/posts.rb
FactoryBot.define do
  factory :post do
    title { Faker::Lorem.sentence }
    body { Faker::Lorem.paragraphs(number: 3).join("\n\n") }
    user
    status { :draft }

    trait :published do
      status { :published }
      published_at { Time.current }
    end
  end
end
```

**Convention:** Fixtures are faster (loaded once) and simpler. Factories are more flexible and explicit. Use fixtures for stable reference data (roles, settings) and factories for test-specific data. Pick one as the primary approach and be consistent.

### System Tests with Capybara

System tests exercise the full stack including JavaScript (Turbo, Stimulus):

```ruby
# test/system/posts_test.rb (Minitest)
require "application_system_test_case"

class PostsTest < ApplicationSystemTestCase
  test "creating a new post" do
    sign_in users(:admin)

    visit new_post_path
    fill_in "Title", with: "My New Post"
    fill_in "Body", with: "This is the content of my new post."
    select "Published", from: "Status"
    click_button "Create Post"

    assert_text "Post was successfully created"
    assert_text "My New Post"
  end

  test "inline editing post title with Turbo Frame" do
    sign_in users(:admin)
    post = posts(:published_post)

    visit post_path(post)
    within "#post_title" do
      click_link "Edit"
      fill_in "Title", with: "Updated Title"
      click_button "Save"
    end

    assert_text "Updated Title"
    assert_no_selector "form"  # frame replaced with display mode
  end
end

# RSpec equivalent
RSpec.describe "Posts", type: :system do
  before { driven_by(:selenium_chrome_headless) }

  it "creates a new post" do
    sign_in create(:user, :admin)

    visit new_post_path
    fill_in "Title", with: "My New Post"
    fill_in "Body", with: "Content here."
    click_button "Create Post"

    expect(page).to have_text("Post was successfully created")
  end
end
```

### Test Configuration

```ruby
# test/test_helper.rb
ENV["RAILS_ENV"] ||= "test"
require_relative "../config/environment"
require "rails/test_help"

module ActiveSupport
  class TestCase
    # Run tests in parallel with specified workers
    parallelize(workers: :number_of_processors)

    # Setup all fixtures in test/fixtures/*.yml
    fixtures :all

    # Add more helper methods to be used by all tests here
  end
end

# test/application_system_test_case.rb
require "test_helper"

class ApplicationSystemTestCase < ActionDispatch::SystemTestCase
  driven_by :selenium, using: :headless_chrome, screen_size: [1400, 900]
end
```

### Request Tests (Integration)

```ruby
# test/controllers/posts_controller_test.rb
class PostsControllerTest < ActionDispatch::IntegrationTest
  setup do
    @user = users(:admin)
    sign_in @user
  end

  test "GET /posts returns success" do
    get posts_url
    assert_response :success
  end

  test "POST /posts creates a post" do
    assert_difference("Post.count") do
      post posts_url, params: { post: { title: "New", body: "Content" } }
    end
    assert_redirected_to post_url(Post.last)
  end

  test "POST /posts with invalid data renders errors" do
    post posts_url, params: { post: { title: "", body: "" } }
    assert_response :unprocessable_entity
  end

  test "DELETE /posts/:id requires authentication" do
    sign_out
    delete post_url(posts(:published_post))
    assert_redirected_to new_session_url
  end

  # Turbo Stream response test
  test "POST /posts with Turbo Stream format" do
    post posts_url,
      params: { post: { title: "New", body: "Content" } },
      as: :turbo_stream

    assert_response :success
    assert_match "turbo-stream", response.body
  end
end
```

---

## 7. Hotwire Patterns

### Turbo Drive

Turbo Drive intercepts all link clicks and form submissions, replacing full page loads with fetch requests that swap the `<body>`. It is enabled by default in Rails 8.

```erb
<%# Opt out for specific links %>
<%= link_to "External", "https://example.com", data: { turbo: false } %>

<%# Opt out for specific forms %>
<%= form_with(url: legacy_path, data: { turbo: false }) do |f| %>
  <%# This form does a full page submit %>
<% end %>

<%# Advance vs. replace navigation %>
<%= link_to "Show", post_path(@post) %>  <%# default: advance (pushes history) %>
<%= link_to "Tab", tab_path, data: { turbo_action: "replace" } %>  <%# replace (no history entry) %>
```

### Turbo Frames

Turbo Frames scope navigation to a specific region of the page:

```erb
<%# app/views/posts/show.html.erb %>
<turbo-frame id="post_header">
  <h1><%= @post.title %></h1>
  <%= link_to "Edit", edit_post_path(@post) %>
</turbo-frame>

<div class="mt-8">
  <%= @post.body %>
</div>

<%# app/views/posts/edit.html.erb %>
<turbo-frame id="post_header">
  <%= form_with(model: @post) do |f| %>
    <%= f.text_field :title, class: "text-2xl font-bold w-full" %>
    <%= f.submit "Save", class: "btn btn-primary" %>
  <% end %>
</turbo-frame>
```

Clicking "Edit" inside the frame replaces only the frame content with the edit form. The rest of the page remains untouched.

#### Lazy Loading Frames

Load content asynchronously after the page renders:

```erb
<%# Main page — frame loads content on connect %>
<turbo-frame id="recent_comments" src="<%= post_comments_path(@post) %>" loading="lazy">
  <p class="text-gray-500 animate-pulse">Loading comments...</p>
</turbo-frame>

<%# Response from /posts/:id/comments wraps content in matching frame %>
<turbo-frame id="recent_comments">
  <% @comments.each do |comment| %>
    <%= render comment %>
  <% end %>
</turbo-frame>
```

#### Frame Navigation — Breaking Out

Target `_top` to break out of a frame and navigate the full page:

```erb
<turbo-frame id="search_results">
  <% @results.each do |result| %>
    <%# This link navigates the whole page, not just the frame %>
    <%= link_to result.title, post_path(result), data: { turbo_frame: "_top" } %>
  <% end %>
</turbo-frame>
```

### Turbo Streams

Turbo Streams perform targeted DOM updates — append, prepend, replace, update, remove, before, after:

```ruby
# app/controllers/comments_controller.rb
class CommentsController < ApplicationController
  def create
    @comment = @post.comments.build(comment_params)
    @comment.user = Current.user

    if @comment.save
      respond_to do |format|
        format.turbo_stream  # renders create.turbo_stream.erb
        format.html { redirect_to @post }
      end
    else
      render :new, status: :unprocessable_entity
    end
  end
end
```

```erb
<%# app/views/comments/create.turbo_stream.erb %>
<%= turbo_stream.append "comments" do %>
  <%= render @comment %>
<% end %>

<%= turbo_stream.update "comment_count" do %>
  <%= @post.comments.count %> comments
<% end %>

<%= turbo_stream.update "new_comment_form" do %>
  <%= render "comments/form", comment: Comment.new %>
<% end %>
```

#### Turbo Stream Broadcasts (Real-Time)

Broadcast updates to all connected clients via Action Cable:

```ruby
# app/models/comment.rb
class Comment < ApplicationRecord
  belongs_to :post
  belongs_to :user

  # Broadcast to all viewers of this post
  after_create_commit -> {
    broadcast_append_to(
      post,
      target: "comments",
      partial: "comments/comment",
      locals: { comment: self }
    )
  }

  after_destroy_commit -> {
    broadcast_remove_to(post)
  }
end
```

```erb
<%# app/views/posts/show.html.erb — subscribe to broadcasts %>
<%= turbo_stream_from @post %>

<div id="comments">
  <%= render @post.comments %>
</div>
```

#### Turbo Morph (Rails 8+)

Turbo 8 introduces morphing for page refreshes — updates only the changed DOM nodes instead of replacing the entire body:

```erb
<%# app/views/layouts/application.html.erb %>
<head>
  <meta name="turbo-refresh-method" content="morph">
  <meta name="turbo-refresh-scroll" content="preserve">
</head>
```

```ruby
# Broadcast a page refresh (morphs the page for all viewers)
class Post < ApplicationRecord
  after_update_commit -> {
    broadcast_refresh_to(self)
  }
end
```

### Stimulus Controllers

Stimulus provides JavaScript behavior tied to HTML via `data-*` attributes:

```javascript
// app/javascript/controllers/search_controller.js
import { Controller } from "@hotwired/stimulus"

export default class extends Controller {
  static targets = ["input", "results"]
  static values = {
    url: String,
    debounce: { type: Number, default: 300 }
  }

  connect() {
    // Called when controller is connected to DOM
  }

  search() {
    clearTimeout(this.timeout)
    this.timeout = setTimeout(() => {
      this.performSearch()
    }, this.debounceValue)
  }

  async performSearch() {
    const query = this.inputTarget.value
    if (query.length < 2) {
      this.resultsTarget.innerHTML = ""
      return
    }

    const response = await fetch(`${this.urlValue}?q=${encodeURIComponent(query)}`, {
      headers: { Accept: "text/vnd.turbo-stream.html" }
    })
    const html = await response.text()
    this.resultsTarget.innerHTML = html
  }

  disconnect() {
    clearTimeout(this.timeout)
  }
}
```

```erb
<%# Usage in HTML %>
<div data-controller="search"
     data-search-url-value="<%= search_posts_path %>"
     data-search-debounce-value="250">
  <input type="search"
         data-search-target="input"
         data-action="input->search#search"
         placeholder="Search posts..."
         class="input input-bordered w-full">
  <div data-search-target="results" class="mt-4"></div>
</div>
```

#### Stimulus Targets

Targets are named element references:

```javascript
// Declaration
static targets = ["input", "output", "counter"]

// Generated methods (for target named "input"):
// this.inputTarget       — first matching element (throws if missing)
// this.inputTargets      — all matching elements (array)
// this.hasInputTarget    — boolean check
```

#### Stimulus Actions

Actions bind DOM events to controller methods:

```erb
<%# Format: event->controller#method %>
<button data-action="click->dropdown#toggle">Toggle</button>

<%# Multiple actions %>
<input data-action="input->search#query focus->search#expand blur->search#collapse">

<%# Keyboard events %>
<input data-action="keydown.enter->form#submit keydown.escape->form#cancel">

<%# Window/document events %>
<div data-action="click@window->dropdown#close resize@window->layout#adjust">
```

#### Stimulus Values

Values are typed, reactive data attributes:

```javascript
static values = {
  url: String,           // data-controller-url-value
  count: Number,         // data-controller-count-value
  open: Boolean,         // data-controller-open-value
  items: Array,          // data-controller-items-value (JSON)
  config: Object         // data-controller-config-value (JSON)
}

// Callback when value changes
countValueChanged() {
  this.counterTarget.textContent = this.countValue
}
```

### Turbo Native (Mobile)

Turbo Native lets you wrap your Rails app in a native iOS/Android shell:

```ruby
# Detect Turbo Native requests
class ApplicationController < ActionController::Base
  private

  def turbo_native_app?
    request.user_agent.to_s.match?(/Turbo Native/)
  end
  helper_method :turbo_native_app?
end
```

```erb
<%# Conditionally hide web-only chrome %>
<% unless turbo_native_app? %>
  <nav class="bg-white shadow">
    <%# Web navigation — native app uses its own nav %>
  </nav>
<% end %>
```

---

## 8. Background Jobs

### Active Job Interface

Active Job provides a unified API regardless of the queue backend:

```ruby
# app/jobs/sync_contact_job.rb
class SyncContactJob < ApplicationJob
  queue_as :default
  retry_on StandardError, wait: :polynomially_longer, attempts: 5
  discard_on ActiveJob::DeserializationError

  def perform(user)
    ExternalService::ContactSync.call(user: user)
  end
end

# Enqueue
SyncContactJob.perform_later(user)                    # async
SyncContactJob.set(wait: 5.minutes).perform_later(user)  # delayed
SyncContactJob.set(queue: :critical).perform_later(user)  # specific queue
```

### Solid Queue (Rails 8 Default)

Solid Queue uses the database as the job backend — no Redis required:

```ruby
# config/application.rb (Rails 8 default)
config.active_job.queue_adapter = :solid_queue

# config/solid_queue.yml
default: &default
  dispatchers:
    - polling_interval: 1
      batch_size: 500
  workers:
    - queues: "*"
      threads: 5
      processes: 1
      polling_interval: 0.1

development:
  <<: *default

production:
  <<: *default
  workers:
    - queues: "critical"
      threads: 3
      processes: 1
      polling_interval: 0.1
    - queues: "default,low"
      threads: 5
      processes: 2
      polling_interval: 1
```

```bash
# Run Solid Queue (included in bin/dev via Procfile.dev)
bundle exec rake solid_queue:start
```

### Sidekiq (Alternative)

For high-throughput job processing with Redis:

```ruby
# Gemfile
gem "sidekiq", "~> 7.0"

# config/application.rb
config.active_job.queue_adapter = :sidekiq

# config/initializers/sidekiq.rb
Sidekiq.configure_server do |config|
  config.redis = { url: ENV.fetch("REDIS_URL", "redis://localhost:6379/0") }
end

Sidekiq.configure_client do |config|
  config.redis = { url: ENV.fetch("REDIS_URL", "redis://localhost:6379/0") }
end
```

### Job Testing

```ruby
# Minitest
class SyncContactJobTest < ActiveJob::TestCase
  test "enqueues the job" do
    assert_enqueued_with(job: SyncContactJob, args: [users(:admin)]) do
      SyncContactJob.perform_later(users(:admin))
    end
  end

  test "performs the sync" do
    # Mock or stub the external service
    ExternalService::ContactSync.stub(:call, true) do
      SyncContactJob.perform_now(users(:admin))
    end
  end
end

# RSpec
RSpec.describe SyncContactJob, type: :job do
  it "enqueues the job" do
    expect {
      described_class.perform_later(user)
    }.to have_enqueued_job(described_class).with(user)
  end

  it "retries on failure" do
    expect(described_class.new.reschedule_at(nil, 3)).to be_present
  end
end
```

**Convention:** Test that jobs are enqueued with the correct arguments. Test job behavior (the `perform` method) separately from enqueueing. Use `perform_enqueued_jobs` block in integration tests when you need jobs to execute inline.

---

## 9. Action Cable

### Channel Setup

```ruby
# app/channels/application_cable/connection.rb
module ApplicationCable
  class Connection < ActionCable::Connection::Base
    identified_by :current_user

    def connect
      self.current_user = find_verified_user
    end

    private

    def find_verified_user
      if (user = User.find_by(id: cookies.encrypted[:user_id]))
        user
      else
        reject_unauthorized_connection
      end
    end
  end
end

# app/channels/post_channel.rb
class PostChannel < ApplicationCable::Channel
  def subscribed
    post = Post.find(params[:id])
    stream_for post
  end

  def unsubscribed
    # Cleanup when client disconnects
  end
end
```

### Solid Cable (Rails 8 Default)

Solid Cable uses the database for WebSocket pub/sub — no Redis required:

```ruby
# config/cable.yml
development:
  adapter: solid_cable
  connects_to:
    database:
      writing: cable
  polling_interval: 0.1

production:
  adapter: solid_cable
  connects_to:
    database:
      writing: cable
  polling_interval: 1

test:
  adapter: test
```

### Turbo Streams over Action Cable

The most common Action Cable pattern in Rails 8 is Turbo Stream broadcasts:

```ruby
# app/models/message.rb
class Message < ApplicationRecord
  belongs_to :room
  belongs_to :user

  after_create_commit -> {
    broadcast_append_to(
      room,
      target: "messages",
      partial: "messages/message"
    )
  }

  after_update_commit -> {
    broadcast_replace_to(
      room,
      target: self,      # uses dom_id
      partial: "messages/message"
    )
  }

  after_destroy_commit -> {
    broadcast_remove_to(room)
  }
end
```

```erb
<%# app/views/rooms/show.html.erb %>
<%= turbo_stream_from @room %>  <%# subscribes via Action Cable %>

<div id="messages" class="space-y-4">
  <%= render @room.messages %>
</div>

<%= form_with(model: [@room, Message.new], class: "mt-4") do |f| %>
  <%= f.text_field :body, placeholder: "Type a message...", class: "input w-full" %>
  <%= f.submit "Send", class: "btn btn-primary" %>
<% end %>
```

### Custom Action Cable Channels (Non-Turbo)

For features that need custom WebSocket logic beyond Turbo Streams:

```ruby
# app/channels/presence_channel.rb
class PresenceChannel < ApplicationCable::Channel
  def subscribed
    stream_from "presence_#{params[:room_id]}"
    broadcast_presence("joined")
  end

  def unsubscribed
    broadcast_presence("left")
  end

  private

  def broadcast_presence(action)
    ActionCable.server.broadcast(
      "presence_#{params[:room_id]}",
      { user: current_user.name, action: action }
    )
  end
end
```

```javascript
// app/javascript/channels/presence_channel.js
import { createConsumer } from "@rails/actioncable"

const consumer = createConsumer()

consumer.subscriptions.create(
  { channel: "PresenceChannel", room_id: roomId },
  {
    received(data) {
      console.log(`${data.user} ${data.action}`)
    }
  }
)
```

---

## 10. Seed Data

### Seed File Structure

```ruby
# db/seeds.rb
puts "Seeding database..."

# Idempotent — safe to run multiple times
# Use find_or_create_by for reference data

# 1. Roles / system config
puts "  Creating system settings..."
SystemSetting.find_or_create_by!(key: "site_name") do |s|
  s.value = "My Application"
end

SystemSetting.find_or_create_by!(key: "max_upload_size") do |s|
  s.value = "10485760"  # 10 MB
end

# 2. Admin user
puts "  Creating admin user..."
admin = User.find_or_create_by!(email_address: "admin@example.com") do |u|
  u.name = "Admin User"
  u.password = "password"
  u.role = :super_admin
end

# 3. Sample data (development only)
if Rails.env.development?
  puts "  Creating development sample data..."

  10.times do |i|
    user = User.find_or_create_by!(email_address: "user#{i}@example.com") do |u|
      u.name = Faker::Name.name
      u.password = "password"
      u.role = :member
    end

    3.times do
      Post.find_or_create_by!(
        title: Faker::Lorem.sentence,
        user: user
      ) do |p|
        p.body = Faker::Lorem.paragraphs(number: 5).join("\n\n")
        p.status = [:draft, :published].sample
        p.published_at = Time.current if p.status == "published"
      end
    end
  end

  puts "  Created #{User.count} users and #{Post.count} posts"
end

puts "Seeding complete."
```

### Faker Gem

```ruby
# Gemfile (development and test groups)
group :development, :test do
  gem "faker", "~> 3.0"
end
```

```ruby
# Usage in seeds or factories
Faker::Name.name                    # => "Jane Smith"
Faker::Internet.email               # => "jane.smith@example.com"
Faker::Lorem.paragraph              # => "Quia voluptas sit..."
Faker::Address.full_address         # => "123 Main St, Anytown, US"
Faker::PhoneNumber.phone_number     # => "(555) 123-4567"
Faker::Company.name                 # => "Acme Corp"
Faker::Lorem.sentence(word_count: 5)
Faker::Date.between(from: 1.year.ago, to: Date.today)
```

### Idempotent Seed Patterns

```ruby
# Pattern 1: find_or_create_by (safest)
Category.find_or_create_by!(name: "Technology") do |c|
  c.slug = "technology"
  c.description = "Tech articles"
end

# Pattern 2: upsert_all (bulk, fastest)
categories = [
  { name: "Technology", slug: "technology", created_at: Time.current, updated_at: Time.current },
  { name: "Design", slug: "design", created_at: Time.current, updated_at: Time.current },
  { name: "Business", slug: "business", created_at: Time.current, updated_at: Time.current }
]
Category.upsert_all(categories, unique_by: :slug)

# Pattern 3: Conditional seeding
unless User.exists?(email_address: "admin@example.com")
  User.create!(email_address: "admin@example.com", password: "password", role: :super_admin)
end
```

**Convention:** Seeds must be idempotent. Running `rails db:seed` twice must not fail or create duplicates. Use `find_or_create_by!` or `upsert_all` with unique constraints.

---

## 11. Development Workflow

### bin/dev

Rails 8 ships with `bin/dev` which uses Foreman (or `foreman` alternative) to run multiple processes:

```bash
# bin/dev — starts all development processes
bin/dev
```

```
# Procfile.dev
web: bin/rails server -p 3000
css: bin/rails tailwindcss:watch
queue: bundle exec rake solid_queue:start
```

### Rails Console

The Rails console is the most powerful development tool:

```bash
bin/rails console               # start console
bin/rails console --sandbox     # wrap everything in a transaction (rolls back on exit)

# In console
User.count                      # quick queries
User.where(role: :admin).pluck(:email_address)
Post.published.recent.limit(5).to_sql   # inspect generated SQL
Post.find_by(slug: "hello-world").update!(status: :published)
ActiveRecord::Base.logger = Logger.new(STDOUT)  # see SQL in console

# Reload code without restarting
reload!
```

### Code Quality Tools

```bash
# RuboCop — Ruby style guide enforcement
bundle exec rubocop              # check all files
bundle exec rubocop -A           # auto-fix safely
bundle exec rubocop --only Style # check only style cops

# Brakeman — security static analysis
bundle exec brakeman             # full security scan
bundle exec brakeman -q          # quiet mode (warnings only)
bundle exec brakeman -o report.html  # HTML report

# ERB Lint — template linting
bundle exec erblint app/views/

# Annotate — add schema comments to models
bundle exec annotate --models    # add schema info to model files
bundle exec annotate --routes    # add route info to routes.rb
```

### Gemfile (Development/Test Group)

```ruby
group :development, :test do
  gem "debug", platforms: %i[mri windows], require: "debug/prelude"
  gem "brakeman", require: false
  gem "rubocop-rails-omakase", require: false
  gem "faker"
  gem "factory_bot_rails"  # if using factories
end

group :development do
  gem "web-console"        # in-browser console on error pages
  gem "rack-mini-profiler" # performance profiling
  gem "annotaterb"         # schema annotations on models
  gem "letter_opener"      # preview emails in browser
end

group :test do
  gem "capybara"
  gem "selenium-webdriver"
  gem "simplecov", require: false
end
```

### Common Development Commands

```bash
# Server & processes
bin/dev                                     # start all processes
bin/rails server                            # web server only
bin/rails server -b 0.0.0.0 -p 3000        # bind to all interfaces

# Database
bin/rails db:create                         # create database
bin/rails db:migrate                        # run pending migrations
bin/rails db:rollback                       # undo last migration
bin/rails db:rollback STEP=3                # undo last 3 migrations
bin/rails db:seed                           # run seeds
bin/rails db:reset                          # drop + create + migrate + seed
bin/rails db:schema:load                    # load schema.rb (faster than migrate)

# Generators
bin/rails generate model Post title:string body:text user:references
bin/rails generate controller Posts index show new create edit update destroy
bin/rails generate migration AddSlugToPosts slug:string:uniq
bin/rails generate authentication           # Rails 8 auth scaffold
bin/rails generate scaffold Post title body:text status:integer

# Routes
bin/rails routes                            # all routes
bin/rails routes -g post                    # grep for "post"
bin/rails routes -c posts                   # routes for PostsController

# Console & debugging
bin/rails console                           # interactive console
bin/rails dbconsole                         # database console (psql)
bin/rails runner "puts User.count"          # run one-off scripts

# Assets
bin/rails assets:precompile                 # compile for production
bin/rails tailwindcss:build                 # build Tailwind CSS

# Quality (define as Rake tasks or use directly)
bundle exec rubocop
bundle exec brakeman -q
bundle exec rails test                      # Minitest
bundle exec rspec                           # RSpec
```

### Rake Task Aliases

```ruby
# lib/tasks/quality.rake
namespace :quality do
  desc "Run all quality checks"
  task all: [:rubocop, :brakeman, :test]

  desc "Run RuboCop"
  task :rubocop do
    sh "bundle exec rubocop"
  end

  desc "Run Brakeman security scan"
  task :brakeman do
    sh "bundle exec brakeman -q --no-pager"
  end

  desc "Run tests with coverage"
  task :test do
    sh "COVERAGE=true bundle exec rails test"
  end
end

# CI task
desc "Run full CI pipeline"
task ci: ["quality:all"]
```

---

## 12. Deployment

### Kamal (Rails 8 Default)

Kamal deploys Rails apps via Docker to any server (VPS, cloud, bare metal):

```yaml
# config/deploy.yml
service: myapp
image: myapp/web

servers:
  web:
    hosts:
      - 192.168.1.100
    labels:
      traefik.http.routers.myapp.rule: Host(`myapp.com`)
      traefik.http.routers.myapp.tls.certresolver: letsencrypt
  job:
    hosts:
      - 192.168.1.100
    cmd: bundle exec rake solid_queue:start

registry:
  server: ghcr.io
  username: myuser
  password:
    - KAMAL_REGISTRY_PASSWORD

env:
  clear:
    RAILS_LOG_TO_STDOUT: true
    RAILS_SERVE_STATIC_FILES: true
  secret:
    - RAILS_MASTER_KEY
    - DATABASE_URL
    - REDIS_URL

builder:
  multiarch: false

traefik:
  options:
    publish:
      - "443:443"
    volume:
      - "/letsencrypt/acme.json:/letsencrypt/acme.json"
```

```bash
# Kamal commands
kamal setup                # first deploy — provisions server, deploys app
kamal deploy               # deploy latest code
kamal rollback             # rollback to previous version
kamal app logs             # stream application logs
kamal app exec "bin/rails console"  # remote console
kamal app exec "bin/rails db:migrate"  # run migrations
```

### Dockerfile (Rails 8 Generated)

Rails 8 generates a production-ready Dockerfile:

```dockerfile
# syntax=docker/dockerfile:1
ARG RUBY_VERSION=3.3.0
FROM docker.io/library/ruby:$RUBY_VERSION-slim AS base

WORKDIR /rails

ENV RAILS_ENV="production" \
    BUNDLE_DEPLOYMENT="1" \
    BUNDLE_PATH="/usr/local/bundle" \
    BUNDLE_WITHOUT="development:test"

# Build stage
FROM base AS build

RUN apt-get update -qq && \
    apt-get install --no-install-recommends -y build-essential git libpq-dev

COPY Gemfile Gemfile.lock ./
RUN bundle install && \
    rm -rf ~/.bundle/ "${BUNDLE_PATH}"/ruby/*/cache

COPY . .

RUN SECRET_KEY_BASE_DUMMY=1 ./bin/rails assets:precompile

# Production stage
FROM base

RUN apt-get update -qq && \
    apt-get install --no-install-recommends -y curl libpq5 && \
    rm -rf /var/lib/apt/lists /var/cache/apt/archives

COPY --from=build "${BUNDLE_PATH}" "${BUNDLE_PATH}"
COPY --from=build /rails /rails

RUN groupadd --system --gid 1000 rails && \
    useradd rails --uid 1000 --gid 1000 --create-home --shell /bin/bash && \
    chown -R rails:rails db log storage tmp

USER 1000:1000

ENTRYPOINT ["/rails/bin/docker-entrypoint"]

EXPOSE 3000
CMD ["./bin/rails", "server"]
```

### Render Deployment

```yaml
# render.yaml
services:
  - type: web
    name: myapp
    runtime: ruby
    plan: starter
    buildCommand: bundle install && bin/rails assets:precompile && bin/rails db:migrate
    startCommand: bin/rails server -b 0.0.0.0
    envVars:
      - key: RAILS_MASTER_KEY
        sync: false
      - key: DATABASE_URL
        fromDatabase:
          name: myapp-db
          property: connectionString

databases:
  - name: myapp-db
    plan: starter
    databaseName: myapp_production
```

### Fly.io Deployment

```toml
# fly.toml
app = "myapp"
primary_region = "iad"

[build]

[deploy]
  release_command = "bin/rails db:migrate"

[http_service]
  internal_port = 3000
  force_https = true
  auto_stop_machines = "stop"
  auto_start_machines = true
  min_machines_running = 1

[processes]
  app = "bin/rails server -b 0.0.0.0 -p 3000"
  worker = "bundle exec rake solid_queue:start"
```

```bash
fly launch                 # initial setup
fly deploy                 # deploy
fly ssh console            # remote shell
fly logs                   # stream logs
fly postgres connect       # database console
```

### GitHub Actions CI

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_USER: rails_test
          POSTGRES_PASSWORD: password
          POSTGRES_DB: myapp_test
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    env:
      RAILS_ENV: test
      DATABASE_URL: postgres://rails_test:password@localhost:5432/myapp_test
      RAILS_MASTER_KEY: ${{ secrets.RAILS_MASTER_KEY }}

    steps:
      - uses: actions/checkout@v4
      - uses: ruby/setup-ruby@v1
        with:
          ruby-version: "3.3"
          bundler-cache: true

      - name: Setup database
        run: bin/rails db:schema:load

      - name: Run RuboCop
        run: bundle exec rubocop

      - name: Run Brakeman
        run: bundle exec brakeman -q --no-pager

      - name: Run tests
        run: bundle exec rails test

      - name: Run system tests
        run: bundle exec rails test:system

  deploy:
    needs: test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'

    steps:
      - uses: actions/checkout@v4
      - uses: superfly/flyctl-actions/setup-flyctl@master
      - run: flyctl deploy --remote-only
        env:
          FLY_API_TOKEN: ${{ secrets.FLY_API_TOKEN }}
```

---

## 13. Security

### Strong Parameters

Never trust user input. Whitelist permitted parameters:

```ruby
class PostsController < ApplicationController
  private

  def post_params
    params.require(:post).permit(:title, :body, :status, :category_id, tag_ids: [])
  end
end
```

**Rules:**
- Never use `params.permit!` — it allows everything
- Never pass `params` directly to `create` or `update`
- Whitelist only the attributes the current user is allowed to change
- Separate param methods for different actions if they have different permitted attributes

```ruby
# Different params for different roles
def post_params
  permitted = [:title, :body]
  permitted += [:featured, :pinned] if Current.user.admin?
  params.require(:post).permit(permitted)
end
```

### CSRF Protection

Rails includes CSRF protection by default:

```ruby
class ApplicationController < ActionController::Base
  # Included by default in ActionController::Base
  # protect_from_forgery with: :exception

  # For API controllers that use token auth:
  # skip_forgery_protection
end
```

```erb
<%# Rails automatically includes CSRF token in forms %>
<%= form_with(model: @post) do |f| %>
  <%# CSRF token is automatically included %>
<% end %>

<%# For non-Rails JavaScript, include the meta tag %>
<%= csrf_meta_tags %>
```

Turbo automatically includes the CSRF token in all fetch requests.

### XSS Protection

Rails auto-escapes all output in ERB templates:

```erb
<%# Safe — auto-escaped %>
<%= user.name %>  <%# <script>alert('xss')</script> becomes &lt;script&gt;... %>

<%# Dangerous — raw output, only use for trusted HTML %>
<%= raw @post.body %>          <%# AVOID — use sanitize instead %>
<%= @post.body.html_safe %>    <%# AVOID — use sanitize instead %>

<%# Safe — sanitized HTML %>
<%= sanitize @post.body, tags: %w[p br strong em a ul ol li h2 h3], attributes: %w[href class] %>
```

**Rule:** Never use `raw` or `html_safe` on user-supplied content. Use `sanitize` with an explicit allowlist of tags and attributes.

### SQL Injection Protection

ActiveRecord parameterizes queries by default:

```ruby
# SAFE — parameterized
User.where(email: params[:email])
User.where("name LIKE ?", "%#{User.sanitize_sql_like(params[:query])}%")
User.find_by(id: params[:id])

# DANGEROUS — string interpolation
User.where("email = '#{params[:email]}'")         # SQL injection!
User.where("name LIKE '%#{params[:query]}%'")     # SQL injection!
User.order(params[:sort])                          # SQL injection!

# Safe ordering
ALLOWED_SORT = %w[name created_at updated_at].freeze
sort_column = ALLOWED_SORT.include?(params[:sort]) ? params[:sort] : "created_at"
User.order(sort_column => :desc)
```

### Content Security Policy

```ruby
# config/initializers/content_security_policy.rb
Rails.application.configure do
  config.content_security_policy do |policy|
    policy.default_src :self
    policy.font_src    :self, "https://fonts.gstatic.com"
    policy.img_src     :self, :data, "https:"
    policy.object_src  :none
    policy.script_src  :self
    policy.style_src   :self, :unsafe_inline  # required for Tailwind
    policy.connect_src :self, "wss:"          # required for Action Cable

    # Nonce-based script execution (recommended)
    policy.script_src :self, :strict_dynamic if Rails.env.production?
  end

  # Generate nonces for script tags
  config.content_security_policy_nonce_generator = ->(request) {
    SecureRandom.base64(16)
  }
  config.content_security_policy_nonce_directives = %w[script-src]
end
```

### Brakeman Security Scanner

Brakeman is a static analysis tool for Rails security:

```bash
# Run full scan
bundle exec brakeman

# Run in CI (exit code 0 only if no warnings)
bundle exec brakeman -q --no-pager --exit-on-warn

# Ignore known false positives
bundle exec brakeman -I config/brakeman.ignore
```

Common Brakeman warnings and fixes:

| Warning | Fix |
|---|---|
| SQL Injection | Use parameterized queries, never interpolate |
| Cross-Site Scripting | Use `sanitize`, never `html_safe` on user input |
| Mass Assignment | Use strong parameters, never `permit!` |
| Remote Code Execution | Never `eval`, `send`, or `constantize` user input |
| File Access | Never use user input in file paths without sanitization |
| Redirect | Validate redirect URLs, use `_path` helpers |
| Session Settings | Use secure cookie settings in production |

### Additional Security Headers

```ruby
# config/initializers/secure_headers.rb (or in ApplicationController)
class ApplicationController < ActionController::Base
  before_action :set_security_headers

  private

  def set_security_headers
    response.set_header("X-Content-Type-Options", "nosniff")
    response.set_header("X-Frame-Options", "SAMEORIGIN")
    response.set_header("Referrer-Policy", "strict-origin-when-cross-origin")
    response.set_header("Permissions-Policy", "camera=(), microphone=(), geolocation=()")
  end
end

# config/environments/production.rb
config.force_ssl = true  # HSTS + redirect HTTP to HTTPS
config.ssl_options = { hsts: { subdomains: true, preload: true, expires: 1.year } }
```

### Secrets Management

```ruby
# Rails credentials (encrypted, committed to repo)
bin/rails credentials:edit                    # edit with $EDITOR
bin/rails credentials:edit --environment production  # per-environment

# Access credentials
Rails.application.credentials.secret_key_base
Rails.application.credentials.dig(:aws, :access_key_id)
Rails.application.credentials.dig(:stripe, :secret_key)

# Never commit:
# - config/master.key
# - config/credentials/production.key
# - .env files with real secrets
# - API keys in source code
```

---

## 14. Coverage Enforcement

### SimpleCov Setup

```ruby
# Gemfile (test group)
group :test do
  gem "simplecov", require: false
end

# test/test_helper.rb (or spec/spec_helper.rb) — MUST be first
if ENV["COVERAGE"] || ENV["CI"]
  require "simplecov"

  SimpleCov.start "rails" do
    # Enforce minimum coverage
    minimum_coverage 95
    minimum_coverage_by_file 80

    # Enable branch coverage
    enable_coverage :branch
    primary_coverage :line

    # Track all files even if not loaded by tests
    track_files "app/**/*.rb"

    # Groups for the report
    add_group "Models",      "app/models"
    add_group "Controllers", "app/controllers"
    add_group "Services",    "app/services"
    add_group "Components",  "app/components"
    add_group "Jobs",        "app/jobs"
    add_group "Channels",    "app/channels"
    add_group "Mailers",     "app/mailers"

    # Exclude generated/boilerplate files
    add_filter "/test/"
    add_filter "/spec/"
    add_filter "/config/"
    add_filter "/db/"
    add_filter "app/channels/application_cable/"
    add_filter "app/jobs/application_job.rb"
    add_filter "app/mailers/application_mailer.rb"
    add_filter "app/models/application_record.rb"
  end
end

# Then load rails test helper
require_relative "../config/environment"
require "rails/test_help"
```

### Running Coverage

```bash
# Generate coverage report
COVERAGE=true bundle exec rails test

# Coverage report with RSpec
COVERAGE=true bundle exec rspec

# Coverage in CI (always on)
# Set ENV["CI"] = "true" in GitHub Actions — SimpleCov starts automatically

# View HTML report
open coverage/index.html
```

### CI Coverage Gate

```yaml
# .github/workflows/ci.yml (test step)
- name: Run tests with coverage
  run: COVERAGE=true bundle exec rails test
  env:
    RAILS_ENV: test

- name: Check coverage threshold
  run: |
    # SimpleCov fails the build if minimum_coverage is not met
    # The exit code from the test run reflects this
```

**Convention:** Target 100% line coverage per CLAUDE.md core rules. The `minimum_coverage` in SimpleCov is the hard gate. Branch coverage is tracked but not gated initially — ramp up over time. CI fails if coverage drops below the threshold.

---

## 15. Form Compliance

All forms must pass the 9-dimension audit from `FORM_PATTERNS.md`.

### form_with (Rails Standard)

```erb
<%# Model-backed form — URL, method, and CSRF token are automatic %>
<%= form_with(model: @post, class: "space-y-6") do |f| %>
  <fieldset>
    <legend class="text-lg font-semibold text-gray-900">Post Details</legend>

    <div class="space-y-4 mt-4">
      <div>
        <%= f.label :title, class: "block text-sm font-medium text-gray-700" %>
        <%= f.text_field :title,
            required: true,
            autocomplete: "off",
            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500" %>
        <% if @post.errors[:title].any? %>
          <p class="mt-1 text-sm text-red-600" role="alert">
            <%= @post.errors.full_messages_for(:title).join(", ") %>
          </p>
        <% end %>
      </div>

      <div>
        <%= f.label :body, class: "block text-sm font-medium text-gray-700" %>
        <%= f.text_area :body,
            rows: 8,
            required: true,
            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500" %>
      </div>

      <div>
        <%= f.label :status, class: "block text-sm font-medium text-gray-700" %>
        <%= f.select :status,
            Post.statuses.keys.map { |s| [s.humanize, s] },
            {},
            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500" %>
      </div>
    </div>
  </fieldset>

  <div class="flex justify-end">
    <%= f.submit @post.persisted? ? "Update Post" : "Create Post",
        class: "btn btn-primary",
        data: { turbo_submits_with: "Saving..." } %>
  </div>
<% end %>
```

### Turbo-Aware Forms

Rails forms work with Turbo by default. Key patterns:

```ruby
# Controller — return 422 for validation errors (Turbo requirement)
class PostsController < ApplicationController
  def create
    @post = Post.new(post_params)

    if @post.save
      redirect_to @post, notice: "Post created."
    else
      # IMPORTANT: status must be :unprocessable_entity for Turbo to re-render
      render :new, status: :unprocessable_entity
    end
  end

  def update
    if @post.update(post_params)
      redirect_to @post, notice: "Post updated."
    else
      render :edit, status: :unprocessable_entity
    end
  end
end
```

```erb
<%# Turbo submit indicator %>
<%= f.submit "Save",
    data: { turbo_submits_with: "Saving..." } %>

<%# Disable Turbo for specific form (e.g., file upload to external service) %>
<%= form_with(url: upload_path, data: { turbo: false }) do |f| %>
  <%= f.file_field :file %>
<% end %>

<%# Target a specific Turbo Frame %>
<%= form_with(model: @comment, data: { turbo_frame: "comments" }) do |f| %>
<% end %>
```

### Error Display Pattern

```erb
<%# app/views/shared/_form_errors.html.erb %>
<% if record.errors.any? %>
  <div class="rounded-md bg-red-50 p-4 mb-6" role="alert" aria-live="polite">
    <div class="flex">
      <svg class="h-5 w-5 text-red-400" fill="currentColor" viewBox="0 0 20 20">
        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
      </svg>
      <div class="ml-3">
        <h3 class="text-sm font-medium text-red-800">
          <%= pluralize(record.errors.count, "error") %> prevented this from being saved:
        </h3>
        <ul class="mt-2 list-disc list-inside text-sm text-red-700">
          <% record.errors.full_messages.each do |message| %>
            <li><%= message %></li>
          <% end %>
        </ul>
      </div>
    </div>
  </div>
<% end %>

<%# Usage %>
<%= render "shared/form_errors", record: @post %>
```

### Inline Field Errors

```erb
<%# Helper for consistent field error display %>
<%# app/helpers/form_helper.rb %>
module FormHelper
  def field_error(record, attribute)
    return unless record.errors[attribute].any?

    content_tag(:p, class: "mt-1 text-sm text-red-600", role: "alert") do
      record.errors.full_messages_for(attribute).join(", ")
    end
  end

  def field_classes(record, attribute, base_classes)
    if record.errors[attribute].any?
      "#{base_classes} border-red-500 focus:border-red-500 focus:ring-red-500"
    else
      "#{base_classes} border-gray-300 focus:border-blue-500 focus:ring-blue-500"
    end
  end
end
```

### Accessibility Requirements

| Requirement | Implementation |
|---|---|
| Labels | Every input has a visible `<label>` with `for` attribute |
| Error announcements | Error summaries use `role="alert"` and `aria-live="polite"` |
| Required fields | Use `required` attribute and visual indicator |
| Optional fields | Mark with "(optional)" text after the label |
| Autocomplete | Set `autocomplete` attribute on name, email, phone, address fields |
| Touch targets | Minimum 44x44px (48px recommended) for buttons and interactive elements |
| Focus management | After error, focus the error summary or first errored field |
| Keyboard navigation | All form controls reachable and operable via keyboard |

---

## 16. Anti-Patterns

| # | Anti-Pattern | Do This Instead |
|---|---|---|
| 1 | **Fat models** — 500+ line models with mixed responsibilities | Extract service objects, concerns, and query objects. Models handle validations, associations, and scopes only. |
| 2 | **N+1 queries** — loading associations in loops without eager loading | Use `includes`, `preload`, or `eager_load`. Enable `strict_loading` in development. Use Bullet gem to detect. |
| 3 | **Callback hell** — external side effects (emails, API calls, jobs) in model callbacks | Move side effects to service objects or controllers. Callbacks should only modify the record itself. |
| 4 | **God controller** — controllers with 20+ actions or 500+ lines | Use RESTful resources. Extract non-CRUD actions into separate controllers. Follow the "two actions per controller" heuristic. |
| 5 | **Business logic in controllers** — complex conditionals, multi-model updates in actions | Extract to service objects. Controllers should be thin: authenticate, authorize, call service, respond. |
| 6 | **`params.permit!`** — permitting all parameters | Always whitelist specific parameters with `permit(:field1, :field2)`. |
| 7 | **String interpolation in queries** — `where("email = '#{email}'")` | Use parameterized queries: `where(email: email)` or `where("email = ?", email)`. |
| 8 | **`html_safe` on user input** — trusting user-supplied HTML | Use `sanitize` with an explicit tag/attribute allowlist. |
| 9 | **Skipping `status: :unprocessable_entity`** on validation failure | Turbo requires 422 status to re-render forms. Always `render :new, status: :unprocessable_entity`. |
| 10 | **Default scope abuse** — `default_scope { where(active: true) }` | Use explicit named scopes. Default scopes cause surprising behavior and are hard to override. |
| 11 | **Overriding `initialize`** in ActiveRecord models | Use `after_initialize` callback or attribute defaults in the migration. `initialize` does not work as expected with AR. |
| 12 | **Storing state in class variables** — `@@counter` or class-level mutable state | Use database, cache, or instance variables. Class variables are shared across requests in multi-threaded servers. |
| 13 | **Missing database indexes** — queries on unindexed columns | Add indexes for all foreign keys, columns in `where` clauses, unique constraints, and sort columns. |
| 14 | **Missing database constraints** — relying only on ActiveRecord validations | Add `null: false`, `unique: true`, and foreign key constraints at the database level. Validations alone have race conditions. |
| 15 | **Rolling your own auth** — custom session/token management | Use Rails 8 built-in auth generator or Devise. Auth is too important to get wrong. |
| 16 | **Monolithic seed files** — 1000-line `seeds.rb` with no structure | Break seeds into separate files by domain. Use idempotent patterns (`find_or_create_by!`). |
| 17 | **Testing implementation, not behavior** — testing private methods or internal state | Test public interfaces and outcomes. Test what the code does, not how it does it. |
| 18 | **`rescue Exception`** — catching all exceptions including `SystemExit` and `SignalException` | Use `rescue StandardError` or specific exception classes. |
| 19 | **Deploying without migration safety** — renaming columns, removing columns without backfill | Use `strong_migrations` gem. Deploy in phases: add new column, backfill, update code, remove old column. |
| 20 | **Ignoring Turbo** — adding `data-turbo="false"` everywhere instead of working with Turbo | Learn Turbo patterns. Forms need 422 on errors. Use Turbo Frames and Streams for partial updates. |
| 21 | **jQuery in a Hotwire app** — mixing jQuery and Stimulus | Use Stimulus controllers for all JavaScript behavior. jQuery is redundant with Turbo + Stimulus. |
| 22 | **Putting secrets in code or environment files committed to git** — `.env` with real API keys | Use Rails encrypted credentials (`bin/rails credentials:edit`). Never commit unencrypted secrets. |
| 23 | **`sleep` in tests** — waiting for async behavior with `sleep 2` | Use Capybara's built-in waiting (`assert_text`, `have_selector`). They automatically retry. |
| 24 | **Massive partials** — 200-line partial files with complex logic | Extract to ViewComponents or Phlex components. Partials should be small and logic-free. |
| 25 | **`before_action` abuse** — loading resources in `before_action` for actions that do not need them | Use `before_action` with `only:` or `except:`. Or load resources in the action that needs them. |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a Rails patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:rails&title=[Rails]%20)**

Use the `patterns:rails` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
