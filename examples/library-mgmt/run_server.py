#!/usr/bin/env python3
"""
Library Management System — Runnable Python Backend
Transpiled from UniLang (.uniL) backend files for runtime execution.
Serves both the REST API and the frontend static files.
"""

import json
import os
import random
from datetime import date, timedelta

import numpy as np
import pandas as pd
from faker import Faker
from flask import Flask, request, jsonify, send_from_directory
from flask_cors import CORS
from sklearn.ensemble import RandomForestClassifier, GradientBoostingClassifier
from sklearn.model_selection import train_test_split, cross_val_score
from sklearn.preprocessing import LabelEncoder, StandardScaler
from sklearn.metrics import classification_report, accuracy_score
import joblib

# ═══════════════════════════════════════════════════════════════════
# Data Generation (from generate_data.uniL)
# ═══════════════════════════════════════════════════════════════════

fake = Faker()
Faker.seed(42)
random.seed(42)
np.random.seed(42)

TOTAL_BOOKS = 10000
TOTAL_REVIEWS = 2000
TOTAL_USERS = 500

CATEGORIES = [
    "Fiction", "Non-Fiction", "Science", "Technology", "History",
    "Biography", "Self-Help", "Children", "Mystery", "Romance",
    "Fantasy", "Philosophy", "Arts", "Cooking", "Travel"
]
CATEGORY_WEIGHTS = [
    0.15, 0.12, 0.08, 0.10, 0.07,
    0.05, 0.08, 0.06, 0.09, 0.06,
    0.05, 0.03, 0.02, 0.02, 0.02
]
PUBLISHERS = [
    "Penguin Random House", "HarperCollins", "Simon & Schuster",
    "Hachette Book Group", "Macmillan Publishers", "Scholastic",
    "Wiley", "O'Reilly Media", "Springer", "Oxford University Press",
    "Cambridge University Press", "MIT Press", "Academic Press",
    "McGraw-Hill", "Pearson Education"
]
LANGUAGES = ["English", "Spanish", "French", "German", "Chinese", "Japanese", "Hindi"]
LANGUAGE_WEIGHTS = [0.60, 0.10, 0.08, 0.07, 0.06, 0.05, 0.04]


def generate_book_title(category):
    templates = {
        "Fiction": lambda: fake.sentence(nb_words=random.randint(2, 6))[:-1],
        "Non-Fiction": lambda: f"The {fake.word().title()} of {fake.word().title()}",
        "Science": lambda: f"{fake.word().title()} {random.choice(['Physics', 'Biology', 'Chemistry', 'Genetics', 'Quantum'])}",
        "Technology": lambda: f"{random.choice(['Modern', 'Advanced', 'Practical', 'Essential'])} {random.choice(['Python', 'Java', 'Systems', 'Algorithms', 'Data', 'Cloud', 'AI'])}",
        "History": lambda: f"The {fake.word().title()} {random.choice(['Empire', 'Revolution', 'Dynasty', 'War', 'Era'])}",
        "Biography": lambda: f"{fake.name()}: {random.choice(['A Life', 'The Untold Story', 'Memoirs', 'A Biography'])}",
        "Self-Help": lambda: f"{random.choice(['The Power of', 'How to', 'Mastering', 'The Art of'])} {fake.word().title()}",
        "Children": lambda: f"The {random.choice(['Little', 'Magic', 'Amazing', 'Brave'])} {fake.word().title()}",
        "Mystery": lambda: f"The {fake.word().title()} {random.choice(['Mystery', 'Case', 'Secret', 'Shadow', 'Cipher'])}",
        "Romance": lambda: f"{random.choice(['Love in', 'Hearts of', 'A Season of', 'Forever'])} {fake.city()}",
        "Fantasy": lambda: f"The {fake.word().title()} {random.choice(['Chronicles', 'Realm', 'Kingdom', 'Prophecy', 'Quest'])}",
        "Philosophy": lambda: f"{random.choice(['Beyond', 'On', 'The Nature of'])} {fake.word().title()}",
        "Arts": lambda: f"{random.choice(['The Art of', 'Understanding', 'Mastering'])} {fake.word().title()}",
        "Cooking": lambda: f"The {fake.word().title()} {random.choice(['Kitchen', 'Cookbook', 'Table', 'Feast'])}",
        "Travel": lambda: f"{random.choice(['Wandering', 'Lost in', 'Discovering'])} {fake.country()}"
    }
    return templates.get(category, lambda: fake.sentence(nb_words=4)[:-1])()


def generate_users(n):
    users = []
    for i in range(n):
        users.append({
            "id": f"USR-{i+1:05d}",
            "name": fake.name(),
            "email": fake.email(),
            "memberSince": str(fake.date_between(start_date="-5y", end_date="today"))
        })
    return users


def generate_books(n):
    books = []
    for i in range(n):
        category = random.choices(CATEGORIES, weights=CATEGORY_WEIGHTS, k=1)[0]
        publish_year = int(np.random.normal(2010, 10))
        publish_year = max(1950, min(2026, publish_year))

        total_copies = int(np.random.exponential(3)) + 1
        total_copies = min(total_copies, 20)

        popularity_factor = random.random()
        if popularity_factor > 0.9:
            total_checkouts = int(np.random.normal(80, 20))
        elif popularity_factor > 0.6:
            total_checkouts = int(np.random.normal(30, 15))
        else:
            total_checkouts = int(np.random.exponential(5))
        total_checkouts = max(0, total_checkouts)

        late_factor = random.random()
        if total_checkouts > 0:
            if late_factor > 0.8:
                late_rate = random.uniform(0.3, 0.7)
            elif late_factor > 0.4:
                late_rate = random.uniform(0.05, 0.25)
            else:
                late_rate = random.uniform(0.0, 0.05)
            late_returns = int(total_checkouts * late_rate)
            on_time_returns = total_checkouts - late_returns
        else:
            late_returns = 0
            on_time_returns = 0

        checked_out = min(int(np.random.exponential(1)), total_copies)
        available_copies = total_copies - checked_out

        status = "available"
        if available_copies == 0:
            status = random.choice(["checked_out", "reserved"])
        elif random.random() < 0.01:
            status = random.choice(["lost", "maintenance"])

        page_count = int(np.random.normal(300, 100))
        page_count = max(50, min(1200, page_count))

        book = {
            "id": f"BK-{i+1:06d}",
            "title": generate_book_title(category),
            "author": fake.name(),
            "isbn": fake.isbn13(),
            "publisher": random.choice(PUBLISHERS),
            "publishYear": publish_year,
            "category": category,
            "totalCopies": total_copies,
            "availableCopies": available_copies,
            "status": status,
            "averageRating": 0.0,
            "totalRatings": 0,
            "totalCheckouts": total_checkouts,
            "lateReturns": late_returns,
            "onTimeReturns": on_time_returns,
            "predictionTag": None,
            "addedDate": str(fake.date_between(start_date="-3y", end_date="today")),
            "description": fake.paragraph(nb_sentences=3),
            "pageCount": page_count,
            "language": random.choices(LANGUAGES, weights=LANGUAGE_WEIGHTS, k=1)[0],
            "lateReturnRate": round(late_returns / max(1, late_returns + on_time_returns), 4)
        }
        books.append(book)
    return books


def generate_reviews(books, users, n):
    reviews = []
    checkout_counts = [b["totalCheckouts"] for b in books]
    total_co = sum(checkout_counts) or 1
    book_weights = [c / total_co + 0.0001 for c in checkout_counts]
    reviewed_books = random.choices(books, weights=book_weights, k=n)

    review_templates = [
        "Great book! Really enjoyed reading it.",
        "Not what I expected, but decent overall.",
        "A masterpiece. Everyone should read this.",
        "Average read. Nothing special.",
        "Could not put it down! Highly recommend.",
        "Boring and slow. Wouldn't recommend.",
        "Interesting perspective. Worth a read.",
        "The author's best work yet.",
        "A bit too long, but the ending was worth it.",
        "Changed my perspective on the subject.",
        "Well-written but the plot was predictable.",
        "An absolute page-turner!",
        "Disappointing. Expected more from this author.",
        "Perfect for a weekend read.",
        "Technical and dense, but very informative."
    ]

    for i in range(n):
        book = reviewed_books[i]
        user = random.choice(users)
        rating = round(float(np.clip(np.random.normal(3.8, 1.0), 1.0, 5.0)), 1)
        review = {
            "id": f"RV-{i+1:06d}",
            "bookId": book["id"],
            "userId": user["id"],
            "userName": user["name"],
            "rating": rating,
            "comment": random.choice(review_templates) + " " + fake.sentence(),
            "reviewDate": str(fake.date_between(start_date="-2y", end_date="today"))
        }
        reviews.append(review)
    return reviews


def apply_ratings_to_books(books, reviews):
    book_ratings = {}
    for review in reviews:
        bid = review["bookId"]
        if bid not in book_ratings:
            book_ratings[bid] = []
        book_ratings[bid].append(review["rating"])

    for book in books:
        ratings = book_ratings.get(book["id"], [])
        if ratings:
            book["averageRating"] = round(sum(ratings) / len(ratings), 2)
            book["totalRatings"] = len(ratings)
    return books


def generate_all_data():
    print("Generating library data...")
    print(f"  Books: {TOTAL_BOOKS}")
    print(f"  Users: {TOTAL_USERS}")
    print(f"  Reviews: {TOTAL_REVIEWS}")

    print("\n[1/4] Generating users...")
    users = generate_users(TOTAL_USERS)
    print("[2/4] Generating books...")
    books = generate_books(TOTAL_BOOKS)
    print("[3/4] Generating reviews...")
    reviews = generate_reviews(books, users, TOTAL_REVIEWS)
    print("[4/4] Applying ratings to books...")
    books = apply_ratings_to_books(books, reviews)

    categories = {}
    for book in books:
        cat = book["category"]
        categories[cat] = categories.get(cat, 0) + 1

    rated = [b["averageRating"] for b in books if b["averageRating"] > 0]
    avg_rating = np.mean(rated) if rated else 0
    avg_checkouts = np.mean([b["totalCheckouts"] for b in books])

    print(f"\n=== Data Generation Summary ===")
    print(f"Total books:        {len(books)}")
    print(f"Total users:        {len(users)}")
    print(f"Total reviews:      {len(reviews)}")
    print(f"Avg rating:         {avg_rating:.2f}")
    print(f"Avg checkouts:      {avg_checkouts:.1f}")

    return {"books": books, "users": users, "reviews": reviews,
            "metadata": {"totalBooks": len(books), "totalUsers": len(users),
                         "totalReviews": len(reviews),
                         "generatedAt": str(date.today()), "version": "1.0"}}


def save_data(data, output_dir):
    os.makedirs(output_dir, exist_ok=True)
    for key in ["books", "users", "reviews", "metadata"]:
        with open(os.path.join(output_dir, f"{key}.json"), "w") as f:
            json.dump(data[key], f, indent=2)
    print(f"Data saved to {output_dir}/")


# ═══════════════════════════════════════════════════════════════════
# ML Prediction Engine (from prediction_engine.uniL)
# ═══════════════════════════════════════════════════════════════════

def extract_features(books):
    features = []
    for book in books:
        total_returns = book["lateReturns"] + book["onTimeReturns"]
        late_rate = book["lateReturns"] / max(1, total_returns)
        rating_component = book["averageRating"] * 0.3
        checkout_component = min(book["totalCheckouts"] / 100.0, 1.0) * 0.5
        review_component = min(book["totalRatings"] / 50.0, 1.0) * 0.2
        popularity_score = rating_component + checkout_component + review_component
        book_age = 2026 - book["publishYear"]

        features.append({
            "totalCheckouts": book["totalCheckouts"],
            "lateReturns": book["lateReturns"],
            "onTimeReturns": book["onTimeReturns"],
            "lateReturnRate": late_rate,
            "averageRating": book["averageRating"],
            "totalRatings": book["totalRatings"],
            "totalCopies": book["totalCopies"],
            "availableCopies": book["availableCopies"],
            "pageCount": book["pageCount"],
            "publishYear": book["publishYear"],
            "bookAge": book_age,
            "popularityScore": popularity_score,
            "checkoutsPerCopy": book["totalCheckouts"] / max(1, book["totalCopies"]),
            "ratingsPerCheckout": book["totalRatings"] / max(1, book["totalCheckouts"]),
            "isEnglish": 1 if book["language"] == "English" else 0,
        })
    return pd.DataFrame(features)


def generate_labels(books):
    labels = []
    for book in books:
        total_returns = book["lateReturns"] + book["onTimeReturns"]
        late_rate = book["lateReturns"] / max(1, total_returns)
        checkouts = book["totalCheckouts"]
        rating = book["averageRating"]

        if checkouts > 40 and rating > 3.5:
            labels.append("most_likely_booked")
        elif late_rate > 0.25 and checkouts > 10:
            labels.append("late_return")
        elif late_rate < 0.10 and checkouts > 15:
            labels.append("on_time_return")
        else:
            labels.append("less_likely_booked")
    return labels


class PredictionEngine:
    def __init__(self):
        self.scaler = StandardScaler()
        self.label_encoder = LabelEncoder()
        self.model = None
        self.boost_model = None
        self.feature_names = None
        self.class_distribution = {}
        self.accuracy = 0.0
        self.trained = False

    def train(self, books):
        print("\n=== Training Prediction Engine ===\n")

        print("[1/5] Extracting features...")
        X = extract_features(books)
        self.feature_names = list(X.columns)

        print("[2/5] Generating labels...")
        y = generate_labels(books)
        y_encoded = self.label_encoder.fit_transform(y)

        unique, counts = np.unique(y, return_counts=True)
        self.class_distribution = dict(zip(unique.tolist(), [int(c) for c in counts]))
        print(f"      Class distribution: {self.class_distribution}")

        print("[3/5] Scaling features...")
        X_scaled = self.scaler.fit_transform(X)

        print("[4/5] Training models...")
        X_train, X_test, y_train, y_test = train_test_split(
            X_scaled, y_encoded, test_size=0.2, random_state=42, stratify=y_encoded)

        self.model = RandomForestClassifier(
            n_estimators=200, max_depth=15, min_samples_split=5,
            min_samples_leaf=2, class_weight="balanced",
            random_state=42, n_jobs=-1)
        self.model.fit(X_train, y_train)

        self.boost_model = GradientBoostingClassifier(
            n_estimators=100, max_depth=8, learning_rate=0.1, random_state=42)
        self.boost_model.fit(X_train, y_train)

        print("[5/5] Evaluating...")
        rf_proba = self.model.predict_proba(X_test)
        gb_proba = self.boost_model.predict_proba(X_test)
        ensemble_proba = (rf_proba + gb_proba) / 2.0
        y_pred = np.argmax(ensemble_proba, axis=1)

        self.accuracy = float(accuracy_score(y_test, y_pred))
        print(f"      Ensemble accuracy: {self.accuracy:.4f}")

        cv_scores = cross_val_score(self.model, X_scaled, y_encoded, cv=5, scoring="accuracy")
        print(f"      5-Fold CV: {cv_scores.mean():.4f} (+/- {cv_scores.std():.4f})")

        self.trained = True
        return {
            "accuracy": round(self.accuracy, 4),
            "cvMean": round(float(cv_scores.mean()), 4),
            "classDistribution": self.class_distribution,
            "booksTagged": len(books)
        }

    def predict_books(self, books):
        if not self.trained:
            raise RuntimeError("Model not trained")

        X = extract_features(books)
        X_scaled = self.scaler.transform(X)

        rf_proba = self.model.predict_proba(X_scaled)
        gb_proba = self.boost_model.predict_proba(X_scaled)
        ensemble_proba = (rf_proba + gb_proba) / 2.0
        predictions = np.argmax(ensemble_proba, axis=1)
        confidences = np.max(ensemble_proba, axis=1)

        tag_names = self.label_encoder.classes_
        for i, book in enumerate(books):
            book["predictionTag"] = tag_names[predictions[i]]
            book["predictionConfidence"] = round(float(confidences[i]), 4)

        tag_counts = {}
        for book in books:
            tag = book["predictionTag"]
            tag_counts[tag] = tag_counts.get(tag, 0) + 1
        print(f"Prediction results: {tag_counts}")
        return books

    def save_model(self, path):
        os.makedirs(path, exist_ok=True)
        joblib.dump(self.model, os.path.join(path, "rf_model.pkl"))
        joblib.dump(self.boost_model, os.path.join(path, "gb_model.pkl"))
        joblib.dump(self.scaler, os.path.join(path, "scaler.pkl"))
        joblib.dump(self.label_encoder, os.path.join(path, "label_encoder.pkl"))
        joblib.dump(self.feature_names, os.path.join(path, "feature_names.pkl"))
        print(f"Model saved to {path}/")

    def load_model(self, path):
        self.model = joblib.load(os.path.join(path, "rf_model.pkl"))
        self.boost_model = joblib.load(os.path.join(path, "gb_model.pkl"))
        self.scaler = joblib.load(os.path.join(path, "scaler.pkl"))
        self.label_encoder = joblib.load(os.path.join(path, "label_encoder.pkl"))
        self.feature_names = joblib.load(os.path.join(path, "feature_names.pkl"))
        self.trained = True
        print(f"Model loaded from {path}/")

    def get_model_info(self):
        return {
            "trained": self.trained,
            "accuracy": round(self.accuracy, 4) if self.trained else None,
            "features": self.feature_names,
            "classes": list(self.label_encoder.classes_) if self.trained else [],
            "classDistribution": self.class_distribution
        }


# ═══════════════════════════════════════════════════════════════════
# Library Service (from library_service.uniL)
# ═══════════════════════════════════════════════════════════════════

class LibraryService:
    def __init__(self):
        self.books_map = {}
        self.reviews_list = []
        self.books_list = []
        self.data_loaded = False

    def load_data(self, data_dir):
        with open(os.path.join(data_dir, "books.json")) as f:
            self.books_list = json.load(f)
        with open(os.path.join(data_dir, "reviews.json")) as f:
            self.reviews_list = json.load(f)

        for book in self.books_list:
            self.books_map[book["id"]] = book

        self.data_loaded = True
        print(f"Loaded {len(self.books_list)} books, {len(self.reviews_list)} reviews")

    def search_books(self, query, page=1, page_size=50):
        q = query.lower()
        results = [b for b in self.books_list
                   if q in b["title"].lower() or q in b["author"].lower()
                   or q in b.get("isbn", "").lower()]
        return self._paginate(results, page, page_size, query=query)

    def filter_books(self, category=None, status=None, prediction_tag=None,
                     min_rating=0, language=None, sort_by="title",
                     sort_order="asc", page=1, page_size=50):
        results = list(self.books_list)
        if category:
            results = [b for b in results if b["category"] == category]
        if status:
            results = [b for b in results if b["status"] == status]
        if prediction_tag:
            results = [b for b in results if b.get("predictionTag") == prediction_tag]
        if min_rating and float(min_rating) > 0:
            results = [b for b in results if b["averageRating"] >= float(min_rating)]
        if language:
            results = [b for b in results if b["language"] == language]

        reverse = sort_order == "desc"
        sort_keys = {
            "rating": lambda b: b["averageRating"],
            "checkouts": lambda b: b["totalCheckouts"],
            "year": lambda b: b["publishYear"],
            "lateRate": lambda b: b.get("lateReturnRate", 0),
        }
        results.sort(key=sort_keys.get(sort_by, lambda b: b["title"].lower()), reverse=reverse)
        return self._paginate(results, page, page_size)

    def get_book_by_id(self, book_id):
        book = self.books_map.get(book_id)
        if not book:
            return None
        reviews = [r for r in self.reviews_list if r["bookId"] == book_id]
        return {"book": book, "reviews": reviews}

    def get_dashboard_stats(self):
        total_books = len(self.books_list)
        available = sum(1 for b in self.books_list if b["status"] == "available")
        checked_out = sum(1 for b in self.books_list if b["status"] == "checked_out")

        ratings = [b["averageRating"] for b in self.books_list if b["averageRating"] > 0]
        checkouts = [b["totalCheckouts"] for b in self.books_list]

        category_stats = {}
        for book in self.books_list:
            cat = book["category"]
            if cat not in category_stats:
                category_stats[cat] = {"count": 0, "totalCheckouts": 0, "totalRatings": 0, "averageRating": 0}
            category_stats[cat]["count"] += 1
            category_stats[cat]["totalCheckouts"] += book["totalCheckouts"]
            category_stats[cat]["totalRatings"] += book["totalRatings"]

        # Compute avg rating per category
        for cat in category_stats:
            cat_books = [b for b in self.books_list if b["category"] == cat and b["averageRating"] > 0]
            if cat_books:
                category_stats[cat]["averageRating"] = round(
                    sum(b["averageRating"] for b in cat_books) / len(cat_books), 2)

        tag_stats = {}
        for book in self.books_list:
            tag = book.get("predictionTag") or "untagged"
            tag_stats[tag] = tag_stats.get(tag, 0) + 1

        top_books = sorted(self.books_list, key=lambda b: -b["totalCheckouts"])[:10]
        rated_books = [b for b in self.books_list if b["totalRatings"] >= 3]
        top_rated = sorted(rated_books, key=lambda b: -b["averageRating"])[:10]
        late_books = sorted(
            [b for b in self.books_list if b["totalCheckouts"] > 5],
            key=lambda b: -b.get("lateReturnRate", 0))[:10]

        return {
            "overview": {
                "totalBooks": total_books,
                "availableBooks": available,
                "checkedOutBooks": checked_out,
                "totalReviews": len(self.reviews_list),
                "averageRating": round(float(np.mean(ratings)), 2) if ratings else 0,
                "totalCheckouts": sum(checkouts),
            },
            "categoryStats": category_stats,
            "predictionTagDistribution": tag_stats,
            "topCheckedOut": [{"title": b["title"], "author": b["author"],
                               "checkouts": b["totalCheckouts"]} for b in top_books],
            "topRated": [{"title": b["title"], "author": b["author"],
                          "rating": b["averageRating"], "reviews": b["totalRatings"]}
                         for b in top_rated],
            "lateReturnHotspots": [{"title": b["title"], "lateRate": b.get("lateReturnRate", 0),
                                    "checkouts": b["totalCheckouts"],
                                    "predictionTag": b.get("predictionTag")}
                                   for b in late_books]
        }

    def get_categories(self):
        return sorted(set(b["category"] for b in self.books_list))

    def get_languages(self):
        return sorted(set(b["language"] for b in self.books_list))

    def update_books_with_predictions(self, predicted_books):
        for book in predicted_books:
            self.books_map[book["id"]] = book
        self.books_list = predicted_books
        print(f"Updated {len(predicted_books)} books with prediction tags.")

    def _paginate(self, results, page, page_size, **extra):
        total = len(results)
        start = (page - 1) * page_size
        end = start + page_size
        resp = {
            "books": results[start:end],
            "total": total,
            "page": page,
            "pageSize": page_size,
            "totalPages": max(1, (total + page_size - 1) // page_size),
        }
        resp.update(extra)
        return resp


# ═══════════════════════════════════════════════════════════════════
# Flask Server (from server.uniL)
# ═══════════════════════════════════════════════════════════════════

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
DATA_DIR = os.path.join(BASE_DIR, "backend", "data", "generated")
MODEL_DIR = os.path.join(BASE_DIR, "backend", "ml", "model")
FRONTEND_DIR = os.path.join(BASE_DIR, "frontend", "public")

app = Flask(__name__, static_folder=FRONTEND_DIR)
CORS(app)

library_service = LibraryService()
prediction_engine = PredictionEngine()
ml_ready = False


# ── Serve frontend ────────────────────────────────────────────────

@app.route("/")
def index():
    return send_from_directory(FRONTEND_DIR, "index.html")


@app.route("/<path:path>")
def static_files(path):
    # Try frontend/public first, then frontend root for src/ paths
    full = os.path.join(FRONTEND_DIR, path)
    if os.path.isfile(full):
        return send_from_directory(FRONTEND_DIR, path)
    # Handle ../src/ relative paths from index.html
    frontend_root = os.path.join(BASE_DIR, "frontend")
    alt = os.path.join(frontend_root, path)
    if os.path.isfile(alt):
        return send_from_directory(frontend_root, path)
    return "Not found", 404


# ── API Routes ────────────────────────────────────────────────────

@app.route("/api/health")
def health():
    return jsonify({
        "status": "healthy",
        "dataLoaded": library_service.data_loaded,
        "mlReady": ml_ready,
        "books": len(library_service.books_list) if library_service.data_loaded else 0,
    })


@app.route("/api/admin/generate-data", methods=["POST"])
def admin_generate_data():
    try:
        data = generate_all_data()
        save_data(data, DATA_DIR)
        library_service.load_data(DATA_DIR)
        return jsonify({"status": "success", "metadata": data["metadata"]})
    except Exception as e:
        return jsonify({"status": "error", "message": str(e)}), 500


@app.route("/api/admin/train-model", methods=["POST"])
def admin_train_model():
    global ml_ready
    try:
        if not library_service.data_loaded:
            return jsonify({"error": "Data not loaded"}), 400
        metrics = prediction_engine.train(library_service.books_list)
        predicted = prediction_engine.predict_books(library_service.books_list)
        library_service.update_books_with_predictions(predicted)
        prediction_engine.save_model(MODEL_DIR)
        ml_ready = True
        return jsonify({"status": "success", **metrics})
    except Exception as e:
        import traceback
        traceback.print_exc()
        return jsonify({"status": "error", "message": str(e)}), 500


@app.route("/api/dashboard")
def dashboard():
    if not library_service.data_loaded:
        return jsonify({"error": "Data not loaded"}), 400
    return jsonify(library_service.get_dashboard_stats())


@app.route("/api/books")
def get_books():
    page = request.args.get("page", 1, type=int)
    page_size = request.args.get("pageSize", 50, type=int)
    query = request.args.get("q", None)
    category = request.args.get("category", None)
    status = request.args.get("status", None)
    prediction_tag = request.args.get("predictionTag", None)
    language = request.args.get("language", None)
    sort_by = request.args.get("sortBy", "title")
    sort_order = request.args.get("sortOrder", "asc")

    if query:
        return jsonify(library_service.search_books(query, page, page_size))
    return jsonify(library_service.filter_books(
        category=category, status=status, prediction_tag=prediction_tag,
        language=language, sort_by=sort_by, sort_order=sort_order,
        page=page, page_size=page_size))


@app.route("/api/books/<book_id>")
def get_book(book_id):
    result = library_service.get_book_by_id(book_id)
    if not result:
        return jsonify({"error": "Book not found"}), 404
    return jsonify(result)


@app.route("/api/categories")
def get_categories():
    return jsonify(library_service.get_categories())


@app.route("/api/languages")
def get_languages():
    return jsonify(library_service.get_languages())


@app.route("/api/predictions/stats")
def prediction_stats():
    if not ml_ready:
        return jsonify({"modelTrained": False}), 200
    stats = library_service.get_dashboard_stats()
    info = prediction_engine.get_model_info()
    return jsonify({
        "modelTrained": True,
        "modelType": "Ensemble (RandomForest + GradientBoosting)",
        "accuracy": info.get("accuracy"),
        "trainingSamples": len(library_service.books_list),
        "booksTagged": len(library_service.books_list),
        "tagDistribution": stats["predictionTagDistribution"],
        "highRiskBooks": stats["lateReturnHotspots"],
    })


@app.route("/api/predictions/books")
def prediction_books():
    tag = request.args.get("tag", None)
    page = request.args.get("page", 1, type=int)
    if not tag:
        return jsonify({"error": "tag parameter required"}), 400
    return jsonify(library_service.filter_books(
        prediction_tag=tag, page=page, sort_by="checkouts", sort_order="desc"))


# ═══════════════════════════════════════════════════════════════════
# Bootstrap
# ═══════════════════════════════════════════════════════════════════

def initialize():
    global ml_ready
    if os.path.exists(os.path.join(DATA_DIR, "books.json")):
        print("Loading existing data...")
        library_service.load_data(DATA_DIR)
    else:
        print("No data found. Generating fresh dataset...")
        data = generate_all_data()
        save_data(data, DATA_DIR)
        library_service.load_data(DATA_DIR)

    if os.path.exists(os.path.join(MODEL_DIR, "rf_model.pkl")):
        print("Loading pre-trained model...")
        prediction_engine.load_model(MODEL_DIR)
        predicted = prediction_engine.predict_books(library_service.books_list)
        library_service.update_books_with_predictions(predicted)
        ml_ready = True
    else:
        print("No pre-trained model. Training new model...")
        prediction_engine.train(library_service.books_list)
        predicted = prediction_engine.predict_books(library_service.books_list)
        library_service.update_books_with_predictions(predicted)
        prediction_engine.save_model(MODEL_DIR)
        ml_ready = True


if __name__ == "__main__":
    initialize()
    print(f"\n{'='*50}")
    print(f"  Library Management System API")
    print(f"  Running on http://localhost:8080")
    print(f"  ML Prediction Engine: {'Ready' if ml_ready else 'Not Ready'}")
    print(f"{'='*50}\n")
    app.run(host="0.0.0.0", port=8080, debug=False)
